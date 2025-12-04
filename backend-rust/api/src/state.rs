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
//!     // Services will be added in Task 5
//!     HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
//! }
//! ```

/// Application state shared across all request handlers
///
/// This struct contains all services and shared resources needed by the API.
/// It is wrapped in `web::Data` (Arc) by Actix-web and cloned for each request.
///
/// # Future Services (Task 5+)
/// - `data_loader: Arc<DataLoaderService>` - Loads papers and embeddings
/// - `similarity_service: Arc<SimilarityService>` - Computes paper similarity
/// - `tsne_service: Arc<TsneService>` - Computes t-SNE coordinates
/// - `clustering_service: Arc<ClusteringService>` - Performs clustering
#[derive(Clone)]
pub struct AppState {
    // Placeholder for future services
    // Services will be added as they are implemented in subsequent tasks
    _marker: std::marker::PhantomData<()>,
}

impl AppState {
    /// Create a new application state
    ///
    /// This will be expanded in Task 5 to initialize all services.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        // State should be clonable (required for Actix-web)
        let _cloned = state.clone();
    }

    #[test]
    fn test_app_state_default() {
        let _state = AppState::default();
    }
}
