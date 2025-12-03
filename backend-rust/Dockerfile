# Multi-stage Dockerfile for Rust backend
# Stage 1: Builder - compile the Rust binary
FROM rust:1.75-bookworm as builder

WORKDIR /build

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy all crate sources
COPY api/ ./api/
COPY services/ ./services/
COPY models/ ./models/
COPY infrastructure/ ./infrastructure/

# Build the release binary
# This will cache dependencies in Docker layers
RUN cargo build --release --bin miccai-backend-rust

# Stage 2: Runtime - minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /build/target/release/miccai-backend-rust /app/miccai-backend-rust

# Create data directory structure (will be overridden by volume mount in production)
RUN mkdir -p /app/src/data/papers_by_id \
    /app/src/data/embeddings_by_id \
    /app/src/data/cache \
    && chown -R app:app /app

# Switch to non-root user
USER app

# Expose port 8000
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Set default environment variables (can be overridden)
ENV DATA_DIR=/app/src/data \
    SERVER_PORT=8000 \
    RUST_LOG=info

# Run the application
CMD ["/app/miccai-backend-rust"]
