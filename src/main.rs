use worker::Worker;
use master::Master;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let files = vec!["./data/abyss.txt".to_string(), 
                                  "./data/isles.txt".to_string(),
                                  "./data/last.txt".to_string(),
                                  "./data/sierra.txt".to_string(),];
    if args.len() >= 3 {
        let server_type = &args[1];
        let addr = &args[2];
        if server_type == "worker" {
            let worker = Worker::new();
            worker.run(addr.to_string()).await?;
        } else if server_type == "master" {
            if args.len() >= 6 {
                let num_reduces = args[3].parse::<u32>()?;
                let mut timeout: u64 = 60;
                let map_wasm_file = &args[4];
                let reduce_wasm_file = &args[5];
                if args.len() == 7 {
                    timeout = args[6].parse::<u64>()?;
                }
                let map_wasm_bytes = std::fs::read(map_wasm_file)?;
                let reduce_wasm_bytes = std::fs::read(reduce_wasm_file)?;
                let master = Master::new(files, num_reduces, map_wasm_bytes, reduce_wasm_bytes, timeout);
                master.run(addr.to_string()).await?;
            } else {
                Err(anyhow::anyhow!("Usage: master <addr:port> <num_reduces> <map_wasm_file> <reduce_wasm_file> [timeout_sec]"))?;
            }
        } else {
            Err(anyhow::anyhow!("Usage: <server_type> <addr:port> [num_reduces map_wasm_file reduce_wasm_file [timeout_sec]]"))?;
        }
    } else {
        Err(anyhow::anyhow!("Usage: <server_type> <addr:port> [num_reduces map_wasm_file reduce_wasm_file [timeout_sec]]"))?;
    }
    Ok(())
}