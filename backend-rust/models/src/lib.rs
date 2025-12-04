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
//!
//! # Type Mapping Decisions
//!
//! ## Floating Point Precision
//! Python's `float` type is 64-bit (double precision). All Rust models use `f64` for
//! floating-point fields to maintain numerical compatibility:
//! - `PaperSimilarity.similarity_score`: `f64`
//! - `GraphNode.x`, `GraphNode.y`: `Option<f64>`
//! - `GraphEdge.similarity`: `f64`
//!
//! ## Integer Types
//! Python's `int` can be arbitrarily large, but for practical purposes in this domain:
//! - `GraphNode.cluster`: `Option<i32>` - cluster IDs are expected to be small integers
//!
//! ## Field Renaming
//! Some fields use Rust-friendly names with serde rename attributes:
//! - `ExternalLink.link_type` serializes as `"type"` (reserved keyword in Rust)
//! - `Paper.abstract_text` serializes as `"abstract"` (reserved keyword in Rust)
//!
//! ## Optional Fields
//! Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]` to match
//! Python's Pydantic behavior of omitting `None` values from JSON output.
//!
//! ## Validation Strategy
//! Per Design Decision #4, we rely on Rust's type system with serde defaults for
//! validation. Schema compliance is verified through:
//! - Unit tests with real JSON fixtures from the Python backend
//! - Round-trip serialization/deserialization tests
//! - Integration tests (in the `tests/` directory) with captured Python responses

pub mod graph;
pub mod paper;

pub use graph::{GraphData, GraphEdge, GraphNode};
pub use paper::{Author, ExternalLink, Paper, PaperSimilarity};
