//! API layer for MICCAI 2025 Papers Visualization backend
//!
//! This crate provides the HTTP server, routing, CORS configuration, and error handling
//! for the Rust backend. It maps Python's `src/api/` module to Rust.
//!
//! # Module Organization
//! - `routes::papers` - REST API endpoints for papers (maps to Python `papers.py`)
//! - `middleware` - CORS, logging, and other middleware
//! - `error` - Error types and HTTP error responses

pub mod error;
pub mod middleware;
pub mod routes;

pub use error::{ApiError, ApiResult};
