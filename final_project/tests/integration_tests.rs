use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};

use final_project::thread_pool::ThreadPool;
use final_project::processor::process_file;
use final_project::analyzers::*;

#[test]
fn processes_multiple_files() {
    fs::create_dir_all("tmp").unwrap();

    for i in 0..5 {
        let mut f = File::create(format!("tmp/{}.txt", i)).unwrap();
        writeln!(f, "Rust rules").unwrap();
    }

    let pool = ThreadPool::new(4);
    let analyzers: Arc<Vec<Box<dyn Analyzer>>> =
        Arc::new(vec![Box::new(WordCount)]);
    let cancelled = Arc::new(Mutex::new(false));

    for i in 0..5 {
        let a = Arc::clone(&analyzers);
        let c = Arc::clone(&cancelled);

        pool.execute(move || {
            let _ = process_file(format!("tmp/{}.txt", i), a, c);
        });
    }

    pool.shutdown();
    fs::remove_dir_all("tmp").unwrap();
}
