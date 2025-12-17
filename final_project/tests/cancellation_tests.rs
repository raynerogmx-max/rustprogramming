use std::sync::{Arc, Mutex};

use final_project::processor::process_file;
use final_project::analyzers::*;
use final_project::errors::ProcessingError;

#[test]
fn cancellation_works() {
    let analyzers: Arc<Vec<Box<dyn Analyzer>>> =
        Arc::new(vec![Box::new(WordCount)]);

    let cancelled = Arc::new(Mutex::new(true));

    let result = process_file(
        "any.txt".to_string(),
        analyzers,
        cancelled,
    );

    assert!(matches!(
        result.errors.first(),
        Some(ProcessingError::Cancelled)
    ));
}
