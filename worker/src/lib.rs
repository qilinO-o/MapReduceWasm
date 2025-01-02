mod state;
mod wasmmaprt;
mod wasmreducert;
use wasmmaprt::WasmMapRuntime;
use wasmreducert::WasmReduceRuntime;
use utils::*;
use uuid::Uuid;
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio::time::{sleep, Duration};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use anyhow::Context as AnyhowContext;

pub struct Worker {
    worker_id: WorkerId,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            worker_id: Uuid::new_v4().as_u128(),
        }
    }

    pub async fn run(self, server_address: String) -> anyhow::Result<()> {
        init_tracing();
        let dir_path = "./out";
        if !Path::new(dir_path).exists() {
            fs::create_dir(dir_path)?;
        }
        let mut client_transport = tarpc::serde_transport::tcp::connect(server_address, Bincode::default);
        client_transport.config_mut().max_frame_length(usize::MAX);
        let client = CoordinatorRPCClient::new(client::Config::default(), client_transport.await
            .context("Worker new client error")?).spawn();
        
        loop {
            let task_result = client.apply_task(context::current(), self.worker_id).await
                .context("Worker apply for task rpc error")?;
            match task_result {
                TaskResult::Ready(task) => {
                    self.do_task(&client, task).await
                        .context("Worker do task error")?;
                }
                TaskResult::Pending => {
                    tracing::info!("Worker {} get no task available", self.worker_id);
                    sleep(Duration::from_millis(500)).await;
                }
                TaskResult::Completed => {
                    tracing::info!("Worker {} get all tasks completed and shutdown", self.worker_id);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn do_task(&self, client: &CoordinatorRPCClient, task: Task) -> anyhow::Result<()> {
        tracing::info!("Worker {} start to do task {:?}", self.worker_id, task.task_id);
        if task.is_map() {
            self.do_map_task(client, task).await.context("Worker do map task error")?;
        } else {
            self.do_reduce_task(client, task).await.context("Worker do reduce task error")?;
        }
        Ok(())
    }

    async fn do_map_task(&self, client: &CoordinatorRPCClient, task: Task) -> anyhow::Result<()>  {
        tracing::info!("Worker {} start to do map task {:?}", self.worker_id, task.task_id);
        if let TaskContext::MapTask { wasm_bytes, input_filepath, num_reduce } = task.task_context {
            let wasm_bytes = wasm_bytes.unwrap_or_default();
            let input_filepath = input_filepath;
            let cpath = Path::new(&input_filepath);
            let num_reduce = num_reduce;
            // do map task
            let file_content = fs::read_to_string(cpath)
                .context("Worker open source file error")?;
            let mut wasm_map_rt = WasmMapRuntime::new(&wasm_bytes)
                .await
                .context("Worker new wasm map runtime error")?;
            let result = wasm_map_rt.do_map(&input_filepath, &file_content)
                .await
                .context("Map wasm result error")?;
            self.write_map_result(client, task.task_id, num_reduce, &result).await
                .context("Write map result error")?;
        } else {
            tracing::error!("Worker {} get a wrong task {}", self.worker_id, task.task_id);
            return Err(anyhow::anyhow!("Worker get a wrong task"));
        }
        Ok(())
    }

    async fn write_map_result(&self, client: &CoordinatorRPCClient, task_id: TaskId, num_reduce: u32, result: &Vec<(String, String)>) -> anyhow::Result<()> {
        tracing::info!("Start write map result");
        let mut buffers = Vec::new();
        let mut files = Vec::new();
        for i in 0..num_reduce {
            let temp_output_filepath = format!("./out/mr-{}-{}-{}", i, task_id, self.worker_id);
            let cpath = Path::new(&temp_output_filepath);
            let f = fs::File::create(cpath)
                .context("Create output file error")?;
            files.push(temp_output_filepath);
            let writer = BufWriter::new(f);
            buffers.push(writer);
        }
        
        let mut used_reduce_idx: HashSet<u32> = HashSet::new();
        for (key, value) in result {
            let idx = ihash(key) % (num_reduce as u64);
            used_reduce_idx.insert(idx as u32);
            buffers[idx as usize].write_fmt(format_args!("{} {}\n", key, value))
                .context("Write map result(key value) error")?;
        }

        for mut buffer in buffers {
            buffer.flush().context("Flush buffer error")?;
        }

        let commit_status = client.commit_task(context::current(), self.worker_id, task_id, Some(used_reduce_idx.into_iter().collect())).await
            .context("Worker commit task rpc error")?;
        if !commit_status {
            for path in files {
                let cpath = Path::new(&path);
                fs::remove_file(cpath)
                    .context("Remove temp map output file after submission error")?;
            }
        }
        Ok(())
    }

    async fn do_reduce_task(&self, client: &CoordinatorRPCClient, task: Task) -> anyhow::Result<()> {
        tracing::info!("Worker {} start to do reduce task {:?}", self.worker_id, task.task_id);
        if let TaskContext::ReduceTask { wasm_bytes, reduce_idx, mapper_ids } = task.task_context {
            let wasm_bytes = wasm_bytes.unwrap_or_default();
            let reduce_idx = reduce_idx;
            let mapper_ids = mapper_ids;
            // do reduce task
            let mut all_kv: HashMap<String, Vec<String>> = HashMap::new();
            for (t_id, w_id) in mapper_ids {
                let input_filepath = format!("./out/mr-{}-{}-{}", reduce_idx, t_id, w_id);
                let cpath = Path::new(&input_filepath);
                let file_content = fs::read_to_string(cpath)
                    .context("Worker open map result file error")?;
                let mut lines = file_content.lines();
                while let Some(l) = lines.next() {
                    if let Some((k, v)) = l.split_once(' ') {
                        if let Some(vs) = all_kv.get_mut(k) {
                            vs.push(v.to_string());
                        } else {
                            all_kv.insert(k.to_string(), vec![v.to_string()]);
                        }
                    }
                }
            }
            tracing::info!("Shuffle complete");
            let mut all_results = Vec::new();
            let kvnum = all_kv.len();
            let mut kvcnt = 1;
            let mut wasm_reduce_rt = WasmReduceRuntime::new(&wasm_bytes)
                .await
                .context("Worker new wasm reduce runtime error")?;
            for (k, vs) in all_kv.iter() {
                if kvcnt % 1000 == 1 {
                    tracing::info!("Do reduce task for key {} no. {} in {} kvs", k, kvcnt, kvnum);
                }
                kvcnt += 1;
                let result = wasm_reduce_rt.do_reduce(k, vs)
                    .await
                    .context("Reduce wasm result error")?;
                all_results.push(result);
            }
            self.write_reduce_result(client, task.task_id, &all_results).await
                .context("Write reduce result error")?;
        } else {
            tracing::error!("Worker {} get a wrong task {}", self.worker_id, task.task_id);
            return Err(anyhow::anyhow!("Worker get a wrong task"));
        }
        Ok(())
    }

    async fn write_reduce_result(&self, client: &CoordinatorRPCClient, task_id: TaskId, result: &Vec<(String, String)>) -> anyhow::Result<()> {
        tracing::info!("Start write reduce result");
        let temp_output_filepath = format!("./out/mr-{}-{}", task_id, self.worker_id);
        let cpath = Path::new(&temp_output_filepath);
        let f = fs::File::create(cpath)
            .context("Create output file error")?;
        let mut writer = BufWriter::new(f);
        for (key, value) in result {
            writer.write_fmt(format_args!("{} {}\n", key, value))
                .context("Write reduce result(key value) error")?;
        }
        writer.flush().context("Flush buffer error")?;
        let commit_status = client.commit_task(context::current(), self.worker_id, task_id, None).await
            .context("Worker commit task rpc error")?;
        if !commit_status {
            fs::remove_file(cpath)
                .context("Remove temp reduce output file after submission error")?;
        }
        Ok(())
    }
}