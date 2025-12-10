use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::config::Config;
use services::data_loader::DataLoader;
use std::sync::Arc;

mod error;
mod routes;

use routes::health::health_check;
use routes::papers::get_papers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Starting MICCAI Backend Rust");

    // Load configuration from environment variables
    let config = Config::from_env();
    log::info!("Configuration loaded:");
    log::info!("  Papers directory: {}", config.papers_dir.display());
    log::info!("  Embeddings directory: {}", config.embeddings_dir.display());
    log::info!("  Cache directory: {}", config.cache_dir.display());
    log::info!("  Server bind address: {}", config.bind_address());
    log::info!("  CORS allowed origins: {:?}", config.cors_allowed_origins);

    // Initialize DataLoader
    log::info!("Initializing DataLoader...");
    let data_loader = Arc::new(DataLoader::new(
        config.papers_dir.clone(),
        config.embeddings_dir.clone(),
    ));

    // Pre-load paper index to verify data availability
    match data_loader.load_paper_index() {
        Ok(index) => {
            log::info!(
                "Paper index loaded successfully: {} papers available",
                index.dataset_info.total_papers
            );
        }
        Err(e) => {
            log::error!("Failed to load paper index: {}", e);
            log::error!("Please ensure the data directory exists and contains valid data");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to load paper index: {}", e),
            ));
        }
    }

    let bind_address = config.bind_address();
    let cors_origins = config.cors_allowed_origins.clone();

    log::info!("Starting HTTP server on {}", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let mut cors = Cors::default();
        for origin in &cors_origins {
            cors = cors.allowed_origin(origin);
        }
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
            ])
            .max_age(3600);

        App::new()
            // Add logging middleware
            .wrap(Logger::default())
            // Add CORS middleware
            .wrap(cors)
            // Share DataLoader across all handlers
            .app_data(web::Data::new(data_loader.clone()))
            // Register route handlers
            .service(health_check)
            .service(get_papers)
    })
    .bind(&bind_address)?
    .run()
    .await
}
