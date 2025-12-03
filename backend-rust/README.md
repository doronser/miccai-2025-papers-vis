# MICCAI Papers Visualization - Rust Backend

This is the Rust backend implementation for the MICCAI 2025 Papers Visualization project, providing high-performance APIs for paper data, similarity search, and network visualization.

## Project Structure

This workspace contains four crates:

- **`models`** - Domain models (Paper, Author, GraphNode, etc.) with Serde serialization
- **`services`** - Business logic for data loading, similarity, and clustering
- **`api`** - HTTP server with Actix-web (to be implemented)
- **`infrastructure`** - File I/O, caching, and configuration utilities

## Prerequisites

- Rust 1.75.0 or later
- Cargo (comes with Rust)

### Optional Tools

- **clippy** (linter) - Install with: `apt-get install rust-clippy` or via rustup
- **rustfmt** (formatter) - Usually included with Rust installation

## Building

```bash
# Build all crates
cargo build

# Build in release mode (optimized)
cargo build --release

# Build a specific crate
cargo build --package models
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test --package models

# Run tests with output
cargo test -- --nocapture
```

## Linting (Optional)

If clippy is installed:

```bash
# Run clippy linter
cargo clippy

# Run clippy with warnings as errors
cargo clippy -- -D warnings
```

## Development Status

**Milestone 1 - Task 1: ✓ Complete**
- [x] Cargo workspace structure
- [x] Domain models with Serde support
- [x] Comprehensive unit tests (14 tests, 100% pass rate)
- [ ] Data loading services (next task)
- [ ] HTTP API endpoints (next task)

## API Compatibility

The domain models maintain 100% API compatibility with the existing Python/FastAPI backend:
- JSON field names use snake_case (matching Python Pydantic models)
- Optional fields are omitted from JSON when null
- All data types match Python equivalents (List → Vec, Optional → Option, etc.)

## Crate Documentation

### models

Defines all domain models used across the application:

- `Paper` - Research paper with metadata, authors, and links
- `Author` - Author information with optional affiliation/email
- `ExternalLink` - External resources (PDFs, code, etc.)
- `PaperSimilarity` - Similarity scoring between papers
- `GraphNode`, `GraphEdge`, `GraphData` - Network visualization structures

All models use Serde for JSON serialization/deserialization with the following conventions:
- `#[serde(rename_all = "snake_case")]` for field name conversion
- `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- `#[serde(default)]` for fields with default values

Run tests:
```bash
cargo test --package models
```

## License

See LICENSE file in the repository root.
