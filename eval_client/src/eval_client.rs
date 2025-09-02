use std::fs::OpenOptions;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::non_blocking;

mod ads;
mod llm;
mod db;
mod adapters;

pub const NB_ITER: usize = 20000;
pub const ROUNDS: usize = 5;

#[tokio::main]
async fn main() {
    let guard = init_tracing();
    llm::benchmark_llm(NB_ITER, ROUNDS).await;
    db::benchmark_db(NB_ITER, ROUNDS).await;
    ads::benchmark_ads(NB_ITER, ROUNDS).await;
    drop(guard)
}

fn init_tracing() -> WorkerGuard {
    
    let log_path = "benchmark.log";
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(log_path)
      .unwrap();
    let (non_blocking, guard) = non_blocking(file);
    // let env_filter = EnvFilter::new("webserver=info");
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // .with(env_filter)
        // .with_target(false)
        .with_ansi(false)
        .init();
    guard
}
