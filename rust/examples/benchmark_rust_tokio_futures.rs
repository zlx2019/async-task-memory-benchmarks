use std::{env, time::Duration};

/// 
/// Rust benchmark for tokio + futures
/// 
#[tokio::main]
async fn main(){
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();
    println!("startup {num_tasks} task.");
    futures::future::join_all((0..num_tasks)
        .map(|_| tokio::time::sleep(Duration::from_secs(10)))).await;
    println!("shutdown.");
}