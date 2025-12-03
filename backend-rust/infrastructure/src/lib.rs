// Infrastructure crate - file access, caching utilities, configuration

pub mod config;
pub mod file_io;

use thiserror::Error;

/// Infrastructure layer errors
#[derive(Error, Debug)]
pub enum InfrastructureError {
    /// File system I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON deserialization errors
    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    /// NPZ file reading errors (ZIP or NPY format issues)
    #[error("NPZ read error: {0}")]
    NpzReadError(String),

    /// Configuration validation errors
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Type alias for Results in the infrastructure layer
pub type Result<T> = std::result::Result<T, InfrastructureError>;
