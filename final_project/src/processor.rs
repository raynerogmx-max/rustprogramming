use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::types::*;
use crate::errors::*;
use crate::analyzers::Analyzer;

pub fn process_file(
    path: String,
    analyzers: Arc<Vec<Box<dyn Analyzer>>>,
    cancelled: Arc<Mutex<bool>>,
) -> FileAnalysis {
    let start = Instant::now();
    let mut stats = FileStats::default();
    let mut errors = Vec::new();

    if *cancelled.lock().unwrap() {
        errors.push(ProcessingError::Cancelled);
        return FileAnalysis {
            filename: path,
            stats,
            errors,
            processing_time: start.elapsed(),
        };
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            stats.size_bytes = content.len() as u64;
            for analyzer in analyzers.iter() {
                analyzer.analyze(&content, &mut stats);
            }
        }
        Err(e) => {
            errors.push(ProcessingError::IoError {
                path: path.clone(),
                source: e,
            });
        }
    }

    FileAnalysis {
        filename: path,
        stats,
        errors,
        processing_time: start.elapsed(),
    }
}
