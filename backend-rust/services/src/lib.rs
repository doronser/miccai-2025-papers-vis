//! Business logic and data processing services
//!
//! This crate contains the core business logic for the MICCAI papers backend,
//! mapping Python's `src/services/` module to Rust.
//!
//! # Module Organization
//! - `data_loader` - Paper and embedding data loading (maps to Python `data_loader.py`)
//! - `similarity` - Similarity computation and graph generation (maps to Python `similarity.py`)
//! - `tsne_service` - t-SNE coordinate computation (maps to Python `tsne_service.py`)

pub mod data_loader;
pub mod similarity;
pub mod tsne_service;

pub use data_loader::DataLoader;
pub use similarity::SimilarityService;
pub use tsne_service::TSNEService;
