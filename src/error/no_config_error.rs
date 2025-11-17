use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("No config file found at {0}")]
pub struct NoConfigError(pub PathBuf);
