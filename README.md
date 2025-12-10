# MICCAI 2025 Papers Visualization - Rust Backend

This repository contains the Rust migration of the MICCAI 2025 Papers Visualization backend.

Source: https://github.com/doronser/miccai-2025-papers-vis.git
Created: Tue Dec  2 09:44:59 UTC 2025

## Project Structure

```
.
├── Cargo.toml              # Root workspace (for building from repository root)
└── backend-rust/
    ├── Cargo.toml          # Main workspace manifest
    ├── rust-toolchain.toml # Rust version specification (1.75.0)
    ├── models/             # Domain models and data structures
    ├── services/           # Business logic services (placeholder)
    ├── infrastructure/     # File I/O, caching, configuration (placeholder)
    └── api/                # HTTP API server (placeholder)
```

## Current Status

**Milestone 1 - Task 1: Cargo Workspace Setup and Core Domain Models** ✅

This task has established the foundational Rust project structure with all core domain models migrated from Python:

### Completed Components

1. **Workspace Structure**: Four-crate workspace with clear separation of concerns
2. **Domain Models** (from `backend/src/models/paper.py`):
   - `Author` - Author information with optional affiliation and email
   - `ExternalLink` - External resource links (PDF, DOI, etc.)
   - `Paper` - Complete paper metadata
   - `PaperSimilarity` - Paper similarity scoring
   - `GraphNode` - Graph visualization node data
   - `GraphEdge` - Graph edge connections
   - `GraphData` - Complete graph structure

All models include:
- Full Serde serialization/deserialization support
- Comprehensive unit tests (19 tests, all passing)
- Exact JSON schema compatibility with Python Pydantic models

### Build and Test

```bash
# Build the entire workspace
cargo build

# Run all tests
cargo test

# Build release binary
cargo build --release
./target/release/miccai-backend-rust
```

### Dependencies

The workspace currently uses minimal dependencies to ensure compatibility with Rust 1.75.0:
- `serde` & `serde_json` - Serialization
- `anyhow` & `thiserror` - Error handling
- `log` - Logging interface

Heavy dependencies (Actix-web, Tokio, Linfa, NDArray) are commented out in the placeholder crates and will be added in subsequent tasks when implementing actual functionality.

## Next Steps

Future tasks will implement:
- Infrastructure utilities (file I/O, caching, configuration)
- Data loading services
- HTTP server with Actix-web
- Similarity computation
- t-SNE and clustering algorithms
- API endpoint handlers
