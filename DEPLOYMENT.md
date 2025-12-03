# Deployment Guide - Rust Backend

This guide provides instructions for building, running, and deploying the MICCAI 2025 Papers Visualization Rust backend.

## Overview

The application consists of:
- **Backend**: Rust backend with Actix-web (replaces FastAPI)
- **Frontend**: React app served by Nginx (unchanged)
- **Data**: File-based storage for papers, embeddings, and cached results

## Prerequisites

### Local Development
- Rust toolchain (1.75 or later)
- Cargo package manager
- Git

### Docker Deployment
- Docker Engine (v20.10 or later)
- Docker Compose (v2.0 or later)

## Local Development Setup

### 1. Install Rust

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update to latest stable
rustup update stable

# Verify installation
rustc --version
cargo --version
```

### 2. Clone Repository

```bash
git clone https://github.com/your-username/miccai-2025-papers-vis.git
cd miccai-2025-papers-vis
```

### 3. Build the Rust Backend

```bash
# Build in development mode (faster compilation, larger binary)
cargo build

# Build in release mode (optimized, smaller binary)
cargo build --release
```

The compiled binary will be located at:
- Development: `target/debug/miccai-backend-rust`
- Release: `target/release/miccai-backend-rust`

### 4. Configure Environment Variables

Create a `.env` file in the project root or set environment variables:

```bash
# Data directory (default: src/data)
export DATA_DIR=../src/miccai-2025-papers-vis/backend/src/data

# Server port (default: 8000)
export SERVER_PORT=8000

# CORS allowed origins (comma-separated)
export CORS_ORIGINS=http://localhost:3000,http://localhost:5173

# Log level (error, warn, info, debug, trace)
export RUST_LOG=info
```

### 5. Run the Backend Locally

```bash
# Using cargo run (development mode)
cargo run

# Or run the compiled binary directly
./target/release/miccai-backend-rust

# With environment variables inline
DATA_DIR=../src/miccai-2025-papers-vis/backend/src/data \
SERVER_PORT=8000 \
RUST_LOG=info \
cargo run --release
```

The API will be available at `http://localhost:8000/api/papers/`

### 6. Verify the Backend

```bash
# Health check
curl http://localhost:8000/health

# List papers
curl http://localhost:8000/api/papers/

# Get paper by ID
curl http://localhost:8000/api/papers/1
```

## Docker Deployment

### 1. Build Docker Image

```bash
# Build the Rust backend image
docker compose build backend-rust

# Or build directly
docker build -t miccai-backend-rust:latest ./backend-rust
```

### 2. Run with Docker Compose

```bash
# Start only the Rust backend
docker compose up backend-rust

# Start in detached mode
docker compose up -d backend-rust

# View logs
docker compose logs -f backend-rust

# Stop services
docker compose down
```

### 3. Volume Configuration

The `docker-compose.yml` mounts two volumes:

1. **Data Directory** (read-only):
   ```yaml
   - ../src/miccai-2025-papers-vis/backend/src/data:/app/src/data:ro
   ```
   This mounts the source data directory as read-only. Update this path to point to your actual data directory.

2. **Cache Directory** (read-write):
   ```yaml
   - miccai_rust_cache:/app/src/data/cache
   ```
   This is a named volume for storing computed results (t-SNE coordinates, network data).

### 4. Environment Variables

The following environment variables can be configured in `docker-compose.yml`:

| Variable | Default | Description |
|----------|---------|-------------|
| `DATA_DIR` | `/app/src/data` | Base data directory |
| `SERVER_PORT` | `8000` | HTTP server port |
| `CORS_ORIGINS` | `http://localhost:3000,http://localhost:5173` | Comma-separated list of allowed origins |
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |

### 5. Docker Image Size

The multi-stage Dockerfile produces a minimal runtime image:
- Builder stage: ~2.5 GB (includes Rust toolchain)
- Runtime stage: ~80-100 MB (Debian slim + binary only)

### 6. Health Checks

The Docker container includes a health check that runs every 30 seconds:

```bash
# Check container health status
docker inspect miccai-backend-rust | grep -A 10 '"Health"'

# Or with Docker Compose
docker compose ps
```

## Production Deployment

### Option 1: Docker Compose (Recommended)

1. **Update Data Directory Path**

   Edit `docker-compose.yml` and update the volume mount:
   ```yaml
   volumes:
     - /path/to/production/data:/app/src/data:ro
     - miccai_rust_cache:/app/src/data/cache
   ```

2. **Configure Production CORS Origins**

   ```yaml
   environment:
     - CORS_ORIGINS=https://yourdomain.com,https://api.yourdomain.com
   ```

3. **Deploy**

   ```bash
   docker compose up -d backend-rust
   ```

### Option 2: Systemd Service

1. **Build and Install Binary**

   ```bash
   cargo build --release
   sudo cp target/release/miccai-backend-rust /usr/local/bin/
   sudo chmod +x /usr/local/bin/miccai-backend-rust
   ```

2. **Create Systemd Service**

   Create `/etc/systemd/system/miccai-backend-rust.service`:

   ```ini
   [Unit]
   Description=MICCAI Backend API (Rust)
   After=network.target

   [Service]
   Type=simple
   User=www-data
   Group=www-data
   WorkingDirectory=/var/www/miccai-2025-papers-vis
   Environment="DATA_DIR=/var/www/miccai-2025-papers-vis/backend/src/data"
   Environment="SERVER_PORT=8000"
   Environment="CORS_ORIGINS=https://yourdomain.com"
   Environment="RUST_LOG=info"
   ExecStart=/usr/local/bin/miccai-backend-rust
   Restart=always
   RestartSec=10

   [Install]
   WantedBy=multi-user.target
   ```

3. **Start Service**

   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable miccai-backend-rust
   sudo systemctl start miccai-backend-rust
   sudo systemctl status miccai-backend-rust
   ```

### Option 3: Nginx Reverse Proxy

Configure Nginx to proxy requests to the Rust backend:

```nginx
server {
    listen 80;
    server_name yourdomain.com;

    # Frontend
    location / {
        root /var/www/miccai-2025-papers-vis/frontend/dist;
        try_files $uri $uri/ /index.html;
    }

    # Backend API
    location /api {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Timeouts for long-running requests (t-SNE computation)
        proxy_read_timeout 300s;
        proxy_connect_timeout 75s;
    }

    # Health check endpoint
    location /health {
        proxy_pass http://127.0.0.1:8000/health;
    }
}
```

## Data Directory Structure

The Rust backend expects the following directory structure:

```
src/data/
├── papers_by_id/
│   ├── index.json          # Paper index metadata
│   ├── 1.json              # Individual paper files
│   ├── 2.json
│   └── ...
├── embeddings_by_id/
│   ├── 1.npz               # Paper embeddings (NumPy format)
│   ├── 2.npz
│   └── ...
└── cache/
    ├── tsne_coordinates.json    # Cached t-SNE results
    └── network_*.json           # Cached network data
```

**Note:** The Python data pipeline tools (`miccai_parallel_scraper.py`, `paper_parser.py`, `scibert_embeddings.py`) remain in Python and continue to generate this data.

## Environment Variable Reference

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `DATA_DIR` | Path | `src/data` | Base directory for all data files |
| `SERVER_PORT` | u16 | `8000` | HTTP server port (1-65535) |
| `CORS_ORIGINS` | String | `http://localhost:3000,http://localhost:5173` | Comma-separated allowed origins |
| `RUST_LOG` | String | `info` | Log level for the application |

## API Endpoints

The Rust backend preserves all existing REST endpoints:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check endpoint |
| `/api/papers/` | GET | List all papers (with pagination) |
| `/api/papers/{id}` | GET | Get paper by ID |
| `/api/papers/search` | GET | Search papers by query |
| `/api/papers/{id}/similar` | GET | Find similar papers |
| `/api/papers/tsne-coordinates` | GET | Get t-SNE visualization data |
| `/api/papers/clusters/data` | GET | Get clustering data |
| `/api/papers/network/data` | GET | Get network graph data |
| `/api/papers/stats/summary` | GET | Get dataset statistics |
| `/api/papers/clusters/` | GET | List all clusters |
| `/api/papers/{id}/highlight` | GET | Get paper highlights |

## Testing

### Manual Testing

```bash
# Health check
curl http://localhost:8000/health

# List papers with pagination
curl "http://localhost:8000/api/papers/?limit=10"

# Get specific paper
curl http://localhost:8000/api/papers/1

# Search papers
curl "http://localhost:8000/api/papers/search?q=deep+learning"

# Get similar papers
curl "http://localhost:8000/api/papers/1/similar?limit=5"

# Get t-SNE coordinates
curl http://localhost:8000/api/papers/tsne-coordinates

# Get statistics
curl http://localhost:8000/api/papers/stats/summary
```

### Performance Testing

```bash
# Install Apache Bench
sudo apt install apache2-utils

# Test concurrent requests
ab -n 1000 -c 10 http://localhost:8000/api/papers/

# Test with authentication header
ab -n 1000 -c 10 -H "Accept: application/json" http://localhost:8000/api/papers/
```

## Monitoring and Logging

### View Logs

```bash
# Docker Compose logs
docker compose logs -f backend-rust

# Systemd logs
journalctl -u miccai-backend-rust -f

# Filter by log level
journalctl -u miccai-backend-rust | grep ERROR
```

### Log Levels

Set `RUST_LOG` to control verbosity:
- `error`: Only errors
- `warn`: Warnings and errors
- `info`: Informational messages (default)
- `debug`: Detailed debugging information
- `trace`: Very verbose tracing

### Container Metrics

```bash
# View container stats
docker stats miccai-backend-rust

# Inspect container
docker inspect miccai-backend-rust

# Check health status
docker inspect --format='{{.State.Health.Status}}' miccai-backend-rust
```

## Troubleshooting

### Common Issues

#### 1. Data Directory Not Found

**Error:** `Failed to load paper index: No such file or directory`

**Solution:**
- Check that `DATA_DIR` points to the correct directory
- Verify the directory structure matches the expected layout
- Ensure the container has read permissions

```bash
# Check environment variable
docker compose exec backend-rust env | grep DATA_DIR

# Check directory contents
docker compose exec backend-rust ls -la /app/src/data/
```

#### 2. Port Already in Use

**Error:** `Address already in use (os error 48)`

**Solution:**
- Change `SERVER_PORT` to a different port
- Stop the conflicting service

```bash
# Find process using port 8000
lsof -i :8000

# Or with netstat
netstat -tuln | grep 8000
```

#### 3. CORS Errors

**Error:** `Access to fetch at '...' has been blocked by CORS policy`

**Solution:**
- Add the frontend origin to `CORS_ORIGINS`
- Ensure the origin format matches exactly (no trailing slash)

```yaml
environment:
  - CORS_ORIGINS=http://localhost:5173,https://yourdomain.com
```

#### 4. Build Failures

**Error:** `error: could not compile` or `cannot find package`

**Solution:**
- Ensure Cargo.lock is present
- Clear the build cache

```bash
# Clear build cache
cargo clean

# Rebuild
cargo build --release

# For Docker builds
docker compose build --no-cache backend-rust
```

#### 5. Slow Startup

**Symptom:** Container takes a long time to become healthy

**Solution:**
- Check if data files are large and taking time to load
- Increase health check `start_period` in docker-compose.yml
- Monitor logs during startup

```bash
docker compose logs -f backend-rust
```

## Performance Optimization

### 1. Enable Release Mode

Always use `--release` for production builds:

```bash
cargo build --release
```

Release builds are 10-100x faster than debug builds.

### 2. Adjust Worker Threads

Actix-web automatically uses the number of CPU cores. To override:

```rust
// In main.rs (if needed)
HttpServer::new(...)
    .workers(4)  // Set to number of CPU cores
    .bind(...)?
    .run()
```

### 3. Increase File Descriptor Limits

For high-traffic deployments:

```bash
# In systemd service file
[Service]
LimitNOFILE=65536
```

### 4. Enable Caching

The Rust backend automatically caches t-SNE and network computations in `data/cache/`. Ensure this directory is writable.

## Security Considerations

1. **Data Directory Permissions**: Mount as read-only (`:ro`) when possible
2. **Non-root User**: Container runs as non-root `app` user
3. **CORS Configuration**: Only allow trusted origins in production
4. **HTTPS**: Use a reverse proxy (Nginx) with SSL certificates
5. **Firewall**: Only expose necessary ports
6. **Updates**: Keep Rust toolchain and dependencies updated

## Backup and Recovery

### Backup Strategy

1. **Code**: Use Git repository
2. **Data Files**: Regular backups of `src/data/papers_by_id` and `src/data/embeddings_by_id`
3. **Cache**: Can be regenerated, but backup for faster recovery
4. **Docker Volumes**: Backup named volumes

```bash
# Backup Docker volume
docker run --rm -v miccai_rust_cache:/data -v $(pwd):/backup \
  alpine tar czf /backup/cache-backup.tar.gz /data
```

### Disaster Recovery

1. Restore data files to the expected directory structure
2. Rebuild and restart the container
3. Cache will be regenerated on first request if missing

## Migration from Python Backend

### Compatibility

The Rust backend maintains full API compatibility with the Python (FastAPI) backend:
- Same URL paths
- Same query parameters
- Same response schemas
- Same error handling

### Side-by-side Testing

Run both backends simultaneously for comparison:

```bash
# Python backend on port 8000
cd backend && python main.py

# Rust backend on port 8001
SERVER_PORT=8001 cargo run --release

# Compare responses
curl http://localhost:8000/api/papers/ > python.json
curl http://localhost:8001/api/papers/ > rust.json
diff python.json rust.json
```

### Switching Frontends

Update the frontend API base URL:

```bash
# .env.production
VITE_API_BASE_URL=http://localhost:8000/api  # Rust backend
```

## Support and Resources

- **Project Repository**: https://github.com/your-username/miccai-2025-papers-vis
- **Rust Documentation**: https://doc.rust-lang.org/
- **Actix-web Documentation**: https://actix.rs/
- **Docker Documentation**: https://docs.docker.com/

## License

[Your License Here]
