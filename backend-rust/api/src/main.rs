//! Main entry point for MICCAI 2025 Papers Visualization Rust backend

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::get("/")]
async fn root() -> impl Responder {
    info!("Root endpoint accessed");
    HttpResponse::Ok().json(serde_json::json!({
        "message": "MICCAI 2025 Papers Visualization API"
    }))
}

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    info!("Health check endpoint accessed");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting MICCAI 2025 Papers Visualization API");

    // Configure CORS origins
    let cors_origins = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001,http://localhost:5173".to_string());
    
    let origins: Vec<String> = cors_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    info!("CORS origins configured: {:?}", origins);

    // Bind address
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("{}:{}", host, port);

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        // Configure CORS
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        for origin in &origins {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(root)
            .service(health)
            // TODO: Add papers router here in subsequent tasks
            // .service(web::scope("/api").configure(routes::papers::configure))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
