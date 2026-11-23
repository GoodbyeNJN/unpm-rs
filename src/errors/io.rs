use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("failed to {operation} `{path}`: {source}")]
pub struct IoError {
    operation: &'static str,
    path: PathBuf,
    #[source]
    source: io::Error,
}

impl IoError {
    pub fn new(operation: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self {
            operation,
            path: path.into(),
            source,
        }
    }
}
