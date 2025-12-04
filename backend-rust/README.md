# MICCAI 2025 Papers Visualization - Rust Backend

This is the Rust backend implementation for the MICCAI 2025 Papers Visualization system, migrated from the Python FastAPI backend for improved performance.

## Project Structure

This project uses a Cargo workspace with four member crates. The workspace root is at the project root, with Rust backend crates located in `backend-rust/`:

```
project-root/
├── Cargo.toml           # Workspace root configuration
├── backend/             # Python backend (to be replaced)
├── frontend/            # React frontend (unchanged)
└── backend-rust/        # Rust backend implementation
    ├── api/                 # HTTP server, routing, CORS, error handling
    │   ├── src/
    │   │   ├── main.rs      # Binary entry point
    │   │   ├── lib.rs       # API library
    │   │   ├── routes/      # API route handlers
    │   │   ├── middleware/  # Custom middleware
    │   │   └── error.rs     # API error types
    │   └── Cargo.toml
    ├── services/            # Business logic and data processing
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── data_loader.rs    # Paper and embedding loading
    │   │   ├── similarity.rs     # Similarity computation
    │   │   └── tsne_service.rs   # t-SNE dimensionality reduction
    │   └── Cargo.toml
    ├── models/              # Domain models and data structures
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── paper.rs     # Paper, Author, PaperSimilarity
    │   │   └── graph.rs     # GraphNode, GraphEdge, GraphData
    │   └── Cargo.toml
    └── infrastructure/      # File I/O, caching, configuration
        ├── src/
        │   ├── lib.rs
        │   ├── config.rs    # Configuration management
        │   ├── file_io.rs   # File system operations
        │   ├── cache.rs     # Caching utilities
        │   └── error.rs     # Infrastructure error types
        └── Cargo.toml
```

## Module Mapping: Python → Rust

### API Layer (`api/`)

| Python                    | Rust                           | Description                |
|---------------------------|--------------------------------|----------------------------|
| `backend/main.py`         | `api/src/main.rs`              | Application entry point    |
| `src/api/papers.py`       | `api/src/routes/papers.rs`     | Papers API endpoints       |
| FastAPI middleware        | `api/src/middleware.rs`        | CORS, logging middleware   |
| FastAPI error handling    | `api/src/error.rs`             | HTTP error responses       |

### Services Layer (`services/`)

| Python                         | Rust                             | Description                    |
|--------------------------------|----------------------------------|--------------------------------|
| `src/services/data_loader.py`  | `services/src/data_loader.rs`    | Paper and embedding loading    |
| `src/services/similarity.py`   | `services/src/similarity.rs`     | Similarity and graph generation|
| `src/services/tsne_service.py` | `services/src/tsne_service.rs`   | t-SNE coordinate computation   |

### Models Layer (`models/`)

| Python                  | Rust                    | Description                  |
|-------------------------|-------------------------|------------------------------|
| `src/models/paper.py`   | `models/src/paper.rs`   | Paper, Author, ExternalLink  |
| `src/models/paper.py`   | `models/src/graph.rs`   | GraphNode, GraphEdge, GraphData |

### Infrastructure Layer (`infrastructure/`)

| Python Equivalent         | Rust                             | Description                    |
|---------------------------|----------------------------------|--------------------------------|
| Environment variables     | `infrastructure/src/config.rs`   | Configuration management       |
| File I/O utilities        | `infrastructure/src/file_io.rs`  | JSON and NPZ file reading      |
| Caching logic             | `infrastructure/src/cache.rs`    | t-SNE and network caching      |
| N/A                       | `infrastructure/src/error.rs`    | Infrastructure error types     |

### Out of Scope

The following Python modules are **NOT migrated** to Rust as they remain Python-based data pipelines:

- `src/lib/miccai_parallel_scraper.py` - Web scraping tool
- `src/lib/paper_parser.py` - Paper parsing utilities
- `src/lib/scibert_embeddings.py` - Embedding generation

## Technology Stack

### Core Dependencies

- **Web Framework**: Actix-web 4.x
- **Async Runtime**: Tokio 1.x
- **Serialization**: serde, serde_json
- **Logging**: tracing, tracing-subscriber
- **Error Handling**: thiserror, anyhow
- **OpenAPI**: utoipa, utoipa-swagger-ui

### ML & Numerical Computing

- **Linear Algebra**: ndarray
- **Statistics**: ndarray-stats
- **t-SNE**: linfa-reduction (linfa 0.7)
- **Clustering**: linfa-clustering
- **File I/O**: ndarray-npy, zip

### Caching & Concurrency

- **In-Memory Cache**: dashmap
- **Async Traits**: async-trait

## Configuration

The application uses environment variables with fallback defaults (Design Decision #3):

### Environment Variables

| Variable          | Description                        | Default                                              |
|-------------------|------------------------------------|------------------------------------------------------|
| `PAPERS_DIR`      | Path to papers directory           | `./data/papers_by_id/`                               |
| `EMBEDDINGS_DIR`  | Path to embeddings directory       | `./data/embeddings_by_id/`                           |
| `CACHE_DIR`       | Path to cache directory            | `./data/cache/`                                      |
| `HOST`            | Server bind address                | `0.0.0.0`                                            |
| `PORT`            | Server port                        | `8000`                                               |
| `CORS_ORIGINS`    | Comma-separated CORS origins       | `http://localhost:3000,http://localhost:3001,http://localhost:5173` |

## Building and Running

### Prerequisites

- Rust 1.70+ (2021 Edition)
- Cargo

### Build

**Important**: All cargo commands should be run from the project root (not from `backend-rust/`), as the workspace root is located there.

```bash
# Build all crates (run from project root)
cargo build

# Build in release mode for production
cargo build --release
```

### Run

```bash
# Run the development server (from project root)
cargo run --bin miccai-backend-rust

# Run with custom environment variables
PAPERS_DIR=/path/to/papers cargo run --bin miccai-backend-rust
```

### Test

```bash
# Run all tests (from project root)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific crate
cargo test -p models
```

## API Endpoints

All endpoints from the Python backend are preserved:

- `GET /api/papers/` - List papers with pagination
- `GET /api/papers/{id}` - Get paper by ID
- `GET /api/papers/search` - Search papers
- `GET /api/papers/{id}/similar` - Get similar papers
- `GET /api/papers/tsne-coordinates` - Get t-SNE coordinates
- `GET /api/papers/graph/data` - Get graph data (**deprecated**)
- `GET /api/papers/clusters/data` - Get clusters data
- `GET /api/papers/network/data` - Get network data
- `GET /api/papers/stats/summary` - Get dataset statistics
- `GET /api/papers/clusters/` - Get paper clusters
- `GET /api/papers/{id}/highlight` - Get paper highlight data

## API Schema Contract

The Rust backend maintains **strict compatibility** with the Python FastAPI/Pydantic schemas:

- Field names are identical (e.g., `name`, `affiliation`, `email` for `Author`)
- Optional fields use `Option<T>` in Rust, matching nullable fields in Python
- JSON serialization is byte-compatible with the Python backend
- OpenAPI spec is available at `/openapi.json`

## Data Storage

The Rust backend uses the **same file-based storage layout** as the Python backend:

- **Papers**: `{PAPERS_DIR}/{paper_id}.json` + `{PAPERS_DIR}/index.json`
- **Embeddings**: `{EMBEDDINGS_DIR}/{paper_id}.npz`
- **Cache**: `{CACHE_DIR}/tsne_coordinates.json`, `{CACHE_DIR}/network_{paper_id}_{top_k}.json`

## Development Roadmap

This is Task [1] - Workspace Setup. The skeleton structure is in place with placeholder implementations.

### Subsequent Tasks

- **Task 2**: Implement models and data structures
- **Task 3**: Implement data loading and file I/O
- **Task 4**: Implement similarity computation
- **Task 5**: Implement t-SNE service
- **Task 6**: Implement API endpoints
- **Task 7**: Testing and validation
- **Task 8**: Docker containerization

## Design Decisions

### Design Decision #2: Error Handling Strategy

This project uses:
- **`thiserror`** for domain-specific error types (InfraError, ApiError)
- **`anyhow`** for internal service errors with context

### Design Decision #3: Data Directory Path Resolution

Chosen strategy: **Environment variables with fallback defaults** resolved at application startup.

Rationale:
- Most flexible across development, Docker, and production environments
- No hardcoded paths
- Easy to override without code changes
- Documented in `infrastructure/src/config.rs`

## License

MIT OR Apache-2.0
