//! Application state management
//!
//! This module defines the application state that is shared across all request handlers.
//! State is wrapped in `web::Data` (Arc) by Actix-web, providing cheap clones for each request.
//!
//! # Design Decision #5: Application State Management
//!
//! We wrap all services in `Arc<T>` and store them in app state using `web::Data`.
//! Services manage their own interior mutability (e.g., using `DashMap` for caches).
//!
//! This approach provides:
//! - Memory-efficient shared caches across all workers
//! - Clear, ergonomic access patterns in handlers
//! - Thread-safe concurrent access without explicit locks in handlers
//!
//! # Usage
//!
//! ```ignore
//! use actix_web::{web, HttpResponse};
//! use api::state::AppState;
//!
//! async fn handler(state: web::Data<AppState>) -> HttpResponse {
//!     // Access services through the state
//!     let papers = state.data_loader.get_all_papers();
//!     HttpResponse::Ok().json(papers)
//! }
//! ```

use services::DataLoader;
use std::sync::Arc;

/// Application state shared across all request handlers
///
/// This struct contains all services and shared resources needed by the API.
/// It is wrapped in `web::Data` (Arc) by Actix-web and cloned for each request.
///
/// # Services
/// - `data_loader` - Loads papers and embeddings from disk with in-memory caching
///
/// # Future Services
/// - `similarity_service` - Computes paper similarity
/// - `tsne_service` - Computes t-SNE coordinates
/// - `clustering_service` - Performs clustering
#[derive(Clone)]
pub struct AppState {
    /// Data loader service for papers and embeddings
    pub data_loader: Arc<DataLoader>,
}

impl AppState {
    /// Create a new application state with the given DataLoader
    ///
    /// # Arguments
    /// * `data_loader` - The DataLoader service to use
    ///
    /// # Example
    /// ```ignore
    /// use infrastructure::Config;
    /// use services::DataLoader;
    /// use api::state::AppState;
    /// use std::sync::Arc;
    ///
    /// let config = Config::from_env();
    /// let data_loader = Arc::new(DataLoader::new(config));
    /// let state = AppState::new(data_loader);
    /// ```
    pub fn new(data_loader: Arc<DataLoader>) -> Self {
        Self {
            data_loader,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infrastructure::Config;
    use services::DataLoader;

    #[test]
    fn test_app_state_creation() {
        let config = Config::from_env();
        let data_loader = Arc::new(DataLoader::new(config));
        let state = AppState::new(data_loader);
        // State should be clonable (required for Actix-web)
        let _cloned = state.clone();
    }
}
