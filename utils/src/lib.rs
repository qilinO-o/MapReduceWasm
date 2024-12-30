use serde::{Deserialize, Serialize};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::{hash::{DefaultHasher, Hash, Hasher}, u32};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

pub type TaskId = u128;
pub type WorkerId = u128;
// Define the Task struct for RPC
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub task_id: TaskId,
    pub task_type: TaskType,
    pub worker_id: Option<WorkerId>,
    pub task_context: TaskContext,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskContext {
    MapTask { wasm_bytes: Option<Vec<u8>>, input_filepath: String, num_reduce: u32 },
    ReduceTask { wasm_bytes: Option<Vec<u8>>, reduce_idx: u32, mapper_ids: Vec<(TaskId, WorkerId)> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskType {
    Map,
    Reduce,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskResult {
    Ready(Task),
    Pending,
    Completed,
}

impl Task {
    pub fn new(task_id: TaskId, task_type: TaskType, task_context: TaskContext) -> Self {
        Self {
            task_id,
            task_type,
            worker_id: None,
            task_context,
        }
    }

    pub fn is_map(&self) -> bool {
        match self.task_type {
            TaskType::Map => true,
            TaskType::Reduce => false, 
        }
    }

    pub fn is_reduce(&self) -> bool {
        match self.task_type {
            TaskType::Map => false,
            TaskType::Reduce => true,
        }
    }
}

impl TaskContext {
    pub fn new_map(input_filepath: String, num_reduce: u32) -> Self {
        TaskContext::MapTask {
            wasm_bytes:None,
            input_filepath,
            num_reduce,
        }
    }

    pub fn new_reduce(reduce_idx: u32, mapper_ids: Vec<(TaskId, WorkerId)>) -> Self {
        TaskContext::ReduceTask {
            wasm_bytes:None,
            reduce_idx,
            mapper_ids,
        }
    }
}

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug,wasmtime=error,cranelift=error,tarpc=error"))
        .init();
}

#[tarpc::service]
pub trait CoordinatorRPC {
    async fn apply_task(worker_id: WorkerId) -> TaskResult;
    // async fn submit_task(worker_id: WorkerId, task_id: TaskId) -> bool;
    async fn commit_task(worker_id: WorkerId, task_id: TaskId, reduce_idxs: Option<Vec<u32>>) -> bool;
}

pub fn ihash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn merge_files<P: AsRef<Path>>(input_files: &[P], output_file: P) -> io::Result<()> {
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file)?;

    let mut writer = BufWriter::new(&mut output);

    for input_file in input_files {
        let file = File::open(input_file)?;
        let mut reader = BufReader::new(file);
        io::copy(&mut reader, &mut writer)?;
    }
    writer.flush()?;
    Ok(())
}