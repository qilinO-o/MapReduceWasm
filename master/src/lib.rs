use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};
use utils::*;
use uuid::Uuid;
use tarpc::{context, server::{self, Channel}, tokio_serde::formats::Bincode, };
use futures::{future, prelude::*};
use tokio::time::{sleep, Duration};
use anyhow::Context as AnyhowContext;

#[derive(PartialEq)]
enum CoordinatorStatus {
    Map,
    Reduce,
    Done,
}

pub struct Coordinator {
    map_tasks: VecDeque<Task>,
    reduce_tasks: VecDeque<Task>,
    working_tasks: HashMap<TaskId, Task>,
    committed_reduce: Vec<(TaskId, WorkerId)>,
    status: CoordinatorStatus,
}

impl Coordinator {
    pub fn new(file_paths: Vec<String>, num_reduces: u32) -> Self {
        let mut map_tasks = VecDeque::new();
        for file_path in file_paths {
            let task_id = Uuid::new_v4().as_u128();
            let task = Task::new(
                task_id,
                TaskType::Map,
                TaskContext::new_map(file_path,num_reduces)
            );
            map_tasks.push_back(task);
        }
        let mut reduce_tasks = VecDeque::new();
        for i in 0..num_reduces {
            let task_id = Uuid::new_v4().as_u128();
            let task = Task::new(
                task_id,
                TaskType::Reduce,
                TaskContext::new_reduce(i, Vec::new())
            );
            reduce_tasks.push_back(task);
        }
        Self {
            map_tasks,
            reduce_tasks,
            working_tasks: HashMap::new(),
            committed_reduce: Vec::new(),
            status: CoordinatorStatus::Map,
        }
    }

    pub async fn track_task(coordinator: Arc<Mutex<Coordinator>>, task_id: TaskId, timeout: u64) {
        sleep(Duration::from_secs(timeout)).await;
        let mut coordinator = coordinator.lock().unwrap();
        let task = coordinator.working_tasks.remove(&task_id);
        if let Some(mut task) = task {
            task.worker_id = None;
            if task.is_map() {
                coordinator.map_tasks.push_back(task);
            } else {
                coordinator.reduce_tasks.push_back(task);
            }
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct CoordinatorServer {
    coordinator: Arc<Mutex<Coordinator>>,
    // const values below after new()
    map_wasm: Vec<u8>,
    reduce_wasm: Vec<u8>,
    num_reduces: u32,
    num_maps: u32,
    timeout: u64,
}

impl CoordinatorServer {
    pub fn new(coordinator: &Arc<Mutex<Coordinator>>, num_reduces: u32, map_wasm: Vec<u8>, reduce_wasm: Vec<u8>, timeout: u64) -> Self {
        let num_maps = coordinator.lock().unwrap().map_tasks.len() as u32;
        Self {
            coordinator: coordinator.clone(),
            map_wasm,
            reduce_wasm,
            num_reduces,
            num_maps,
            timeout,
        }
    }

    pub fn get_info(&self) -> (u32, u32) {
        (self.num_maps, self.num_reduces)
    }
}

impl CoordinatorRPC for CoordinatorServer {
    async fn apply_task(self, _: context::Context, worker_id: WorkerId) -> TaskResult {
        let mut coordinator = self.coordinator.lock().unwrap();
        let mut next_task = None;
        // transit the map reduce status of the coordinator
        if coordinator.status == CoordinatorStatus::Map && coordinator.map_tasks.is_empty() && coordinator.working_tasks.is_empty() {
            coordinator.status = CoordinatorStatus::Reduce;
        } else if coordinator.status == CoordinatorStatus::Reduce && coordinator.reduce_tasks.is_empty() && coordinator.working_tasks.is_empty() {
            coordinator.status = CoordinatorStatus::Done;
        }

        // find the proper next task according to status
        if coordinator.status == CoordinatorStatus::Map && !coordinator.map_tasks.is_empty() {
            next_task = coordinator.map_tasks.pop_front();
        } else if coordinator.status == CoordinatorStatus::Reduce && !coordinator.reduce_tasks.is_empty() {
            next_task = coordinator.reduce_tasks.pop_front();
        }
        match next_task {
            Some(mut task) => {
                task.worker_id = Some(worker_id);
                coordinator.working_tasks.insert(task.task_id, task.clone());
                match &mut task.task_context {
                    TaskContext::MapTask { wasm_bytes, input_filepath: _, num_reduce: _ } => {
                        *wasm_bytes = Some(self.map_wasm.clone());
                    }
                    TaskContext::ReduceTask { wasm_bytes, reduce_idx: _, mapper_ids: _ } => {
                        *wasm_bytes = Some(self.reduce_wasm.clone());
                    }
                }
                // monitor task execution to avoid worker failure(timeout)
                tokio::spawn(Coordinator::track_task(self.coordinator.clone(), task.task_id, self.timeout));
                tracing::info!("Coordinator gave {:?} task {} to worker {}", task.task_type, task.task_id, worker_id);
                TaskResult::Ready(task)
            },
            None => {
                if coordinator.status == CoordinatorStatus::Done {
                    tracing::info!("Coordinator informed worker {} map reduce completed", worker_id);
                    TaskResult::Completed
                } else {
                    tracing::info!("Coordinator informed worker {} map reduce pending", worker_id);
                    TaskResult::Pending
                }
            }
        }
    }

    // the result file at caller is guaranteed to be finished and saved
    async fn commit_task(self, _: context::Context, worker_id: WorkerId, task_id: TaskId, reduce_idxs: Option<Vec<u32>>) -> bool {
        let mut coordinator = self.coordinator.lock().unwrap();
        let record_task = coordinator.working_tasks.get(&task_id);
        // check if the task is assigned to the worker
        if let Some(t) = record_task {
            // check if the task result is reported by the right worker
            if t.worker_id == Some(worker_id) {
                if t.is_map() {
                    // prepare reduce task
                    for idx in reduce_idxs.unwrap_or_default() {
                        let reduce_task = coordinator.reduce_tasks.get_mut(idx as usize);
                        if let Some(reduce_task) = reduce_task {
                            match &mut reduce_task.task_context {
                                TaskContext::ReduceTask { wasm_bytes: _, reduce_idx: _, mapper_ids } => {
                                    mapper_ids.push((task_id, worker_id));
                                }
                                _ => {}
                            }
                        } else {
                            tracing::info!("Coordinator failed to find reduce task {}", idx);
                        }
                    }
                } else {
                    coordinator.committed_reduce.push((task_id, worker_id));
                }
                // remove the task from working tasks
                coordinator.working_tasks.remove(&task_id);
                tracing::info!("Coordinator received task commit {} from worker {}", task_id, worker_id);
                return true;
            }
        }
        tracing::info!("Coordinator rejected task commit {} from worker {}", task_id, worker_id);
        false
    }
}

pub struct Master {
    file_paths: Vec<String>,
    num_reduces: u32,
    map_wasm: Vec<u8>,
    reduce_wasm: Vec<u8>,
    timeout: u64,
}

impl Master {
    pub fn new(file_paths: Vec<String>, num_reduces: u32, map_wasm: Vec<u8>, reduce_wasm: Vec<u8>, timeout: u64) -> Self {
        Self {
            file_paths,
            num_reduces,
            map_wasm,
            reduce_wasm,
            timeout,
        }
    }

    pub async fn run(self, server_address: String) -> anyhow::Result<()> {
        init_tracing();
        let mut server_transport = tarpc::serde_transport::tcp::listen(&server_address, Bincode::default)
            .await
            .context("Coordinator tcp listen error")?;
        tracing::info!("Coordinator server listening on {}", server_address);
        server_transport.config_mut().max_frame_length(usize::MAX);
        let coordinator = Arc::new(Mutex::new(Coordinator::new(self.file_paths.clone(), self.num_reduces)));
        let guide_coordinator = coordinator.clone();
        let handle = tokio::spawn(
            async move {
            server_transport
                .filter_map(|r| future::ready(r.ok()))
                .map(server::BaseChannel::with_defaults)
                .map(|channel| { 
                    let server = CoordinatorServer::new(
                        &coordinator, 
                        self.num_reduces, 
                        self.map_wasm.clone(), 
                        self.reduce_wasm.clone(),
                        self.timeout);
                    tracing::info!("Coordinator info {:?}", server.get_info());
                    
                    channel.execute(server.serve())
                        .for_each(|response| async move {
                            tokio::spawn(response);
                        })
                })
                .buffer_unordered(10)
                .for_each(|_| async {})
                .await;
        });
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        tracing::info!("Coordinator server started, waiting to check if map reduce is done");
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            tracing::info!("Coordinator checking if map reduce is done");
            let coordinator = guide_coordinator.lock().unwrap();
            if coordinator.status == CoordinatorStatus::Done {
                // waiting for all workers to shutdown gracefully
                let _ = tokio::time::sleep(std::time::Duration::from_secs(2));
                // collect all reduce results to a single file
                let mut committed_files = Vec::new();
                for (task_id, worker_id) in &coordinator.committed_reduce {
                    committed_files.push(format!("./out/mr-{}-{}", task_id, worker_id));
                }
                let mut target_file = String::from("./out/map_reduce_result_");
                target_file.push_str(
                    SystemTime::now()
                    .duration_since(UNIX_EPOCH).unwrap()
                    .as_secs()
                    .to_string().as_str()
                );
                tracing::info!("Coordinator merging committed files to {}", target_file);
                merge_files(&committed_files, target_file)
                    .context("Coordinator merge files error")?;
                for path in committed_files {
                    fs::remove_file(path)
                        .context("Remove committed file after merge error")?;
                }
                break;
            }
        }
        // Graceful shutdown of the server after tasks are completed
        tracing::info!("Coordinator shutting down server");
        handle.abort();
        Ok(())
    }
}