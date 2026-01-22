use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use miccai_2025_papers_vis::config::AppConfig;
use miccai_2025_papers_vis::services::data_loader::DataLoader;
use std::sync::Arc;

/// Root endpoint returning API information
async fn root() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "title": "MICCAI 2025 Papers Visualization API",
        "description": "API for exploring MICCAI 2025 conference papers through interactive graph visualization",
        "version": "1.0.0"
    }))
}

/// Health check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");

    // Display server startup information
    println!("Starting MICCAI 2025 Papers Visualization API");
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("CORS origins: {:?}", config.cors_origins);
    println!("Data directory: {}", config.data_dir);

    // Instantiate DataLoader service
    let data_loader = Arc::new(DataLoader::new(&config));

    // Clone config for use in closure
    let bind_host = config.host.clone();
    let bind_port = config.port;
    let cors_origins = config.cors_origins.clone();

    // Create and start HTTP server
    HttpServer::new(move || {
        // Configure CORS middleware
        let cors = Cors::default()
            .allowed_origin_fn({
                let allowed_origins = cors_origins.clone();
                move |origin, _req_head| {
                    let origin_str = origin.to_str().unwrap_or("");
                    allowed_origins.iter().any(|allowed| allowed == origin_str)
                }
            })
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::from(data_loader.clone()))
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
    })
    .bind((bind_host.as_str(), bind_port))?
    .run()
    .await
}
