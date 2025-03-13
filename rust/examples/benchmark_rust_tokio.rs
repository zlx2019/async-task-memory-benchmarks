use std::{env, time::Duration};
use tokio::time::sleep;

///
/// Rust benchmark for tokio
/// 
#[tokio::main]
async fn main(){
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();
    println!("startup {num_tasks} task.");
    // let mut tasks = Vec::with_capacity(num_tasks);
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..num_tasks {
        tasks.spawn(sleep(Duration::from_secs(10)));
    }
    tasks.join_all().await;
    println!("shutdown");
}