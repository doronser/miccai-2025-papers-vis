//! Infrastructure error types

use thiserror::Error;

/// Result type for infrastructure operations
pub type InfraResult<T> = Result<T, InfraError>;

/// Infrastructure error types
#[derive(Debug, Error)]
pub enum InfraError {
    /// File I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// NPZ file error
    #[error("NPZ error: {0}")]
    Npz(String),
    
    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),
    
    /// File not found
    #[error("File not found: {0}")]
    NotFound(String),
    
    /// Invalid data format
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
}
