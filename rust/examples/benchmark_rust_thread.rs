use std::{env, thread, time::Duration};

///
/// Rust benchmark for thread (os ulimit)
/// 
fn main(){
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();
    let mut tasks = Vec::with_capacity(num_tasks);
    println!("startup {num_tasks} task.");
    for _ in 0..num_tasks {
        let task = thread::spawn(||{
            thread::sleep(Duration::from_secs(10));
        });
        tasks.push(task);
    }
    for task in tasks{
        task.join().unwrap();
    }
}