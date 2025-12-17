use std::fs::{self, File};
use std::io::Write;
use std::sync::{Arc, Mutex};

use final_project::processor::process_file;
use final_project::analyzers::*;

#[test]
fn computes_correct_stats() {
    let path = "test.txt";
    let mut file = File::create(path).unwrap();
    writeln!(file, "Hello Rust").unwrap();

    let analyzers: Arc<Vec<Box<dyn Analyzer>>> =
        Arc::new(vec![Box::new(WordCount), Box::new(LineCount)]);

    let result = process_file(path.to_string(), analyzers, Arc::new(Mutex::new(false)));

    assert_eq!(result.stats.word_count, 2);
    assert_eq!(result.stats.line_count, 1);

    fs::remove_file(path).unwrap();
}
