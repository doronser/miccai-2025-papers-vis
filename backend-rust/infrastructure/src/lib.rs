//! Infrastructure layer for file access, caching, and configuration
//!
//! This crate provides utilities for interacting with the file system,
//! managing configuration, and handling cross-cutting infrastructure concerns.
//!
//! # Module Organization
//! - `config` - Configuration management and environment variable handling
//! - `file_io` - File system operations for papers and embeddings
//! - `cache` - Caching utilities for t-SNE coordinates and network data
//! - `error` - Infrastructure-level error types

pub mod cache;
pub mod config;
pub mod error;
pub mod file_io;

pub use config::Config;
pub use error::{InfraError, InfraResult};
