//=============================================== 
//================ Uriel Angeles ================ 
//=============================================== 

use std::sync::{Arc, Mutex};
use std::fs;

use final_project::thread_pool::ThreadPool;
use final_project::analyzers::*;
use final_project::processor::process_file;
use final_project::progress::ProgressTracker;

fn main() {
    let pool = ThreadPool::new(8);

    let analyzers: Arc<Vec<Box<dyn Analyzer>>> = Arc::new(vec![
        Box::new(WordCount),
        Box::new(LineCount),
        Box::new(CharFrequency),
    ]);

    let cancelled = Arc::new(Mutex::new(false));
    let progress = Arc::new(Mutex::new(ProgressTracker::default()));

    let files: Vec<String> = fs::read_dir("gutenberg_books")
        .unwrap_or_else(|_| panic!("Missing gutenberg_books directory"))
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    for file in files {
        let analyzers = Arc::clone(&analyzers);
        let cancelled = Arc::clone(&cancelled);
        let progress = Arc::clone(&progress);

        pool.execute(move || {
            let result = process_file(file, analyzers, cancelled);

            println!(
                "Processed {} in {:?} (errors: {})",
                result.filename,
                result.processing_time,
                result.errors.len()
            );

            let mut p = progress.lock().unwrap();
            p.update(result.processing_time, !result.errors.is_empty());
        });
    }

    pool.shutdown();

    let p = progress.lock().unwrap();
    println!("\n===== SUMMARY =====");
    println!("Files processed: {}", p.processed);
    println!("Files with errors: {}", p.errors);
    println!("Total processing time: {:?}", p.total_time);

}
