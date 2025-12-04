//! Main entry point for MICCAI 2025 Papers Visualization Rust backend
//!
//! This binary provides the HTTP server using Actix-web with:
//! - Health check and root endpoints
//! - CORS middleware for frontend integration
//! - Structured logging with tracing
//! - Environment-based configuration
//! - Graceful shutdown handling
//!
//! # Environment Variables
//! - `HOST` - Bind address (default: "0.0.0.0")
//! - `PORT` - Bind port (default: "8000")
//! - `CORS_ORIGINS` - Comma-separated list of allowed origins (default: "http://localhost:3000,http://localhost:3001,http://localhost:5173")
//! - `RUST_LOG` - Log level configuration (default: "info")
//!
//! # Example Usage
//! ```bash
//! # Start with defaults
//! cargo run -p api
//!
//! # Start with custom port
//! PORT=8080 cargo run -p api
//!
//! # Start with debug logging
//! RUST_LOG=debug cargo run -p api
//! ```

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{routes, AppState};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Initialize logging using infrastructure crate
    infrastructure::logging::init_logging();

    info!("Starting MICCAI 2025 Papers Visualization API");

    // Parse CORS origins from environment
    let cors_origins = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| {
            "http://localhost:3000,http://localhost:3001,http://localhost:5173".to_string()
        });

    let origins: Vec<String> = cors_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    info!("CORS origins configured: {:?}", origins);

    // Parse bind address from environment
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("{}:{}", host, port);

    info!("Server will bind to {}", bind_addr);

    // Initialize application state
    // Services will be added in Task 5
    let app_state = web::Data::new(AppState::new());
    info!("Application state initialized");

    // Start HTTP server
    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        // Configure CORS middleware
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        // Add each allowed origin
        for origin in &origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            // Add CORS middleware
            .wrap(cors)
            // Add logging middleware
            .wrap(Logger::default())
            // Add application state
            .app_data(app_state.clone())
            // Register root and health endpoints
            .service(routes::root::root)
            .service(routes::health::health)
            // Register papers API routes (currently empty, will be populated in Task 5+)
            .service(
                web::scope("/api")
                    .configure(routes::papers::configure)
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}
