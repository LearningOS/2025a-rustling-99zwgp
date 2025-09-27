// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let mut handles = vec![];
    for _ in 0..10 {
        let status = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // 锁定Mutex以访问和修改数据
            // 锁会在作用域结束时自动释放
            let mut job_status = status.jobs_completed.lock().unwrap(); //先拿锁再修改
            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    // 等所有线程跑完
    for handle in handles {
        handle.join().unwrap();
    }
    // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
    // anything interesting in the output? Do you have to 'join' on all the
    // handles?
    println!("jobs completed {}", status.jobs_completed.lock().unwrap());
}
