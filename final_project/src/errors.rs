use std::io;

#[derive(Debug)]
pub enum ProcessingError {
    IoError {
        path: String,
        source: io::Error,
    },
    Utf8Error {
        path: String,
    },
    Cancelled,
}
