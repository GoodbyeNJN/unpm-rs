use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("no package.json found in or above `{start}`")]
    ProjectNotFound { start: PathBuf },

    #[error("invalid package.json at `{path}`: {reason}")]
    InvalidPackageJson { path: PathBuf, reason: String },
}
