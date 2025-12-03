use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::config::Config;
use log::{error, info};
use services::DataLoader;
use std::io;

mod errors;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting MICCAI 2025 Papers Visualization API");

    // Load configuration
    let config = Config::from_env().unwrap_or_else(|e| {
        error!("Failed to load configuration: {}", e);
        std::process::exit(1);
    });

    info!("Configuration loaded:");
    info!("  Server port: {}", config.server_port);
    info!("  Data directory: {:?}", config.data_dir);
    info!("  CORS origins: {:?}", config.cors_origins);

    // Initialize DataLoader
    info!("Initializing DataLoader...");
    let data_loader = DataLoader::new(config.clone());

    // Preload paper index to catch any errors early
    match data_loader.load_paper_index() {
        Ok(index) => {
            info!(
                "Paper index loaded successfully: {} papers",
                index.dataset_info.total_papers
            );
        }
        Err(e) => {
            error!("Failed to load paper index: {}", e);
            error!("Make sure the DATA_DIR environment variable points to the correct directory");
            error!("Current DATA_DIR: {:?}", config.data_dir);
            std::process::exit(1);
        }
    }

    // Wrap DataLoader in web::Data for sharing across workers
    let data_loader = web::Data::new(data_loader);

    let server_port = config.server_port;
    let cors_origins = config.cors_origins.clone();

    info!("Starting HTTP server on 0.0.0.0:{}", server_port);

    // Create and start the HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        // Add allowed origins
        for origin in &cors_origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(data_loader.clone())
            .configure(routes::papers::configure_routes)
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}
