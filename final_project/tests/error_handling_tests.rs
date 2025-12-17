use std::sync::{Arc, Mutex};

use final_project::processor::process_file;
use final_project::analyzers::*;

#[test]
fn missing_file_returns_error() {
    let analyzers: Arc<Vec<Box<dyn Analyzer>>> =
        Arc::new(vec![Box::new(WordCount)]);

    let result = process_file(
        "nope.txt".to_string(),
        analyzers,
        Arc::new(Mutex::new(false)),
    );

    assert!(!result.errors.is_empty());
}
