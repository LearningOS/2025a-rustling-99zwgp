// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// Arc：原子引用计数（Atomic Reference Counting），用于在多个线程间安全地共享数据的所有权
// Mutex：互斥锁（Mutual Exclusion），确保同一时间只有一个线程能访问共享数据，防止数据竞争
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

//定义共享状态结构
struct JobStatus {
    jobs_completed: u32, //这个结构体用于跟踪已完成的工作数量，jobs_completed 字段会被多个线程修改
}

fn main() {
    // 创建共享状态
    // 首先用 Mutex 包裹 JobStatus，确保线程安全访问
    // 再用 Arc 包裹 Mutex，允许状态在多个线程间共享所有权
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    // 循环创建 10 个线程
    for _ in 0..10 {
        let status_shared = Arc::clone(&status); //通过 Arc::clone 创建新的引用，让每个线程都拥有共享状态的引用
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status_guard = status_shared.lock().unwrap(); //MutexGuard 离开作用域时自动释放锁，避免手动管理锁的复杂性和遗忘释放的风险
            status_guard.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let status_guard = status.lock().unwrap();
        println!("jobs completed {}", status_guard.jobs_completed);
    }
}
