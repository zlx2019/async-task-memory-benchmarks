use std::{env, time::Duration};
use async_std::task;


/// use async_std
#[async_std::main]
async fn main(){
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();
    let mut tasks = Vec::with_capacity(num_tasks);
    println!("startup {num_tasks} task.");
    for _ in 0..num_tasks {
        tasks.push(task::spawn(task::sleep(Duration::from_secs(10))));
    }
    for task in tasks{
        task.await;
    }
}