// Services crate - data loading, similarity, t-SNE, clustering

pub mod data_loader;

use thiserror::Error;

/// Service layer errors
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Infrastructure errors (file I/O, JSON parsing, NPZ reading)
    #[error("Infrastructure error: {0}")]
    InfrastructureError(#[from] infrastructure::InfrastructureError),

    /// Paper not found error
    #[error("Paper not found: {0}")]
    PaperNotFound(String),

    /// Embedding not found error
    #[error("Embedding not found: {0}")]
    EmbeddingNotFound(String),
}

/// Type alias for Results in the service layer
pub type Result<T> = std::result::Result<T, ServiceError>;

// Re-export commonly used types
pub use data_loader::DataLoader;
