//! Infrastructure error types
//!
//! This module implements Design Decision #2: Error Handling and Propagation Pattern.
//! It uses the `thiserror` crate to provide custom error enums with rich context.
//!
//! Error types include:
//! - File I/O errors with path context
//! - JSON parsing errors with file context
//! - NPZ file errors with detailed diagnostics
//! - Cache errors with operation context
//! - Not found errors with missing resource information
//! - Invalid format errors with validation details

use std::path::PathBuf;
use thiserror::Error;

/// Result type for infrastructure operations
pub type InfraResult<T> = Result<T, InfraError>;

/// Infrastructure error types with rich context
#[derive(Debug, Error)]
pub enum InfraError {
    /// File I/O error with path context
    #[error("I/O error accessing {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// JSON parsing error with file context
    #[error("JSON parsing error in {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    /// NPZ file error with detailed diagnostics
    #[error("NPZ file error in {path}: {message}")]
    Npz {
        path: PathBuf,
        message: String,
    },

    /// Cache operation error
    #[error("Cache error for {operation}: {message}")]
    Cache {
        operation: String,
        message: String,
    },

    /// File or resource not found
    #[error("Resource not found: {path}")]
    NotFound {
        path: PathBuf,
    },

    /// Invalid data format with validation details
    #[error("Invalid data format in {path}: {message}")]
    InvalidFormat {
        path: PathBuf,
        message: String,
    },

    /// ZIP archive error
    #[error("ZIP archive error in {path}: {source}")]
    Zip {
        path: PathBuf,
        #[source]
        source: zip::result::ZipError,
    },
}

impl InfraError {
    /// Create a file not found error
    pub fn not_found(path: impl Into<PathBuf>) -> Self {
        Self::NotFound { path: path.into() }
    }

    /// Create an I/O error with path context
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    /// Create a JSON error with path context
    pub fn json(path: impl Into<PathBuf>, source: serde_json::Error) -> Self {
        Self::Json {
            path: path.into(),
            source,
        }
    }

    /// Create an NPZ error with path and message
    pub fn npz(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::Npz {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a cache error
    pub fn cache(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Cache {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Create an invalid format error
    pub fn invalid_format(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a ZIP error with path context
    pub fn zip(path: impl Into<PathBuf>, source: zip::result::ZipError) -> Self {
        Self::Zip {
            path: path.into(),
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_not_found_error() {
        let err = InfraError::not_found("/path/to/file.json");
        assert!(err.to_string().contains("/path/to/file.json"));
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_io_error_with_context() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let err = InfraError::io("/path/to/file.json", io_err);
        assert!(err.to_string().contains("/path/to/file.json"));
        assert!(err.to_string().contains("Access denied"));
    }

    #[test]
    fn test_cache_error() {
        let err = InfraError::cache("load_tsne", "Cache file corrupted");
        assert!(err.to_string().contains("load_tsne"));
        assert!(err.to_string().contains("corrupted"));
    }

    #[test]
    fn test_invalid_format_error() {
        let err = InfraError::invalid_format("/path/to/data.json", "Missing required field 'id'");
        assert!(err.to_string().contains("/path/to/data.json"));
        assert!(err.to_string().contains("Missing required field"));
    }

    #[test]
    fn test_npz_error() {
        let err = InfraError::npz("/path/to/embedding.npz", "Missing 'embedding' array");
        assert!(err.to_string().contains("embedding.npz"));
        assert!(err.to_string().contains("Missing 'embedding' array"));
    }
}
