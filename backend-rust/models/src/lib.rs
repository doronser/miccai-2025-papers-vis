//! Domain models for MICCAI papers backend
//!
//! This crate contains all data structures used across the application,
//! mapping Python's `src/models/` module to Rust.
//!
//! # Module Organization
//! - `paper` - Paper, Author, ExternalLink, PaperSimilarity (maps to Python `paper.py`)
//! - `graph` - GraphNode, GraphEdge, GraphData (maps to Python `paper.py`)
//!
//! # API Schema Contract
//!
//! The Rust `serde` structs MUST use identical field names and optionality as the
//! current FastAPI/Pydantic models:
//! - `Author` – `name`, `affiliation` (Option), `email` (Option)
//! - `ExternalLink` – `type`, `url`, `description` (Option)
//! - `Paper` – `id`, `title`, `abstract`, `authors`, `subject_areas`, `external_links`,
//!   `publication_date` (Option), `raw_data_source` (Option)
//! - `PaperSimilarity` – `paper_id`, `similarity_score`, `paper`
//! - `GraphNode`, `GraphEdge` – as currently produced by FastAPI

pub mod graph;
pub mod paper;

pub use graph::{GraphData, GraphEdge, GraphNode};
pub use paper::{Author, ExternalLink, Paper, PaperSimilarity};
