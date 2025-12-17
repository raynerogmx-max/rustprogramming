use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use final_project::thread_pool::ThreadPool;

#[test]
fn executes_all_jobs() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            *counter.lock().unwrap() += 1;
        });
    }

    pool.shutdown();
    assert_eq!(*counter.lock().unwrap(), 10);
}

#[test]
fn runs_in_parallel() {
    let pool = ThreadPool::new(4);
    let start = std::time::Instant::now();

    for _ in 0..4 {
        pool.execute(|| {
            thread::sleep(Duration::from_millis(200));
        });
    }

    pool.shutdown();
    assert!(start.elapsed() < Duration::from_millis(600));
}
