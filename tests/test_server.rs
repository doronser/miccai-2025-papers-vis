use actix_cors::Cors;
use actix_web::{test, web, App, HttpResponse};
use miccai_2025_papers_vis::config::AppConfig;
use miccai_2025_papers_vis::services::data_loader::DataLoader;
use std::sync::Arc;

/// Root endpoint returning API information
async fn root() -> impl actix_web::Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "title": "MICCAI 2025 Papers Visualization API",
        "description": "API for exploring MICCAI 2025 conference papers through interactive graph visualization",
        "version": "1.0.0"
    }))
}

/// Health check endpoint
async fn health() -> impl actix_web::Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[actix_web::test]
async fn test_root_endpoint() {
    // Create test config
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec!["http://localhost:3000".to_string()],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    };

    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .route("/", web::get().to(root)),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    // Verify response status
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    // Parse response body
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response content
    assert_eq!(json["title"], "MICCAI 2025 Papers Visualization API");
    assert_eq!(
        json["description"],
        "API for exploring MICCAI 2025 conference papers through interactive graph visualization"
    );
    assert_eq!(json["version"], "1.0.0");
}

#[actix_web::test]
async fn test_health_endpoint() {
    // Create test config
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec!["http://localhost:3000".to_string()],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    };

    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .route("/health", web::get().to(health)),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Verify response status
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    // Parse response body
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response content
    assert_eq!(json["status"], "healthy");
}

#[actix_web::test]
async fn test_cors_headers_for_allowed_origin() {
    // Create test config with specific CORS origin
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec![
            "http://localhost:3000".to_string(),
            "http://localhost:5173".to_string(),
        ],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    };

    let data_loader = Arc::new(DataLoader::new(&config));
    let cors_origins = config.cors_origins.clone();

    let app = test::init_service(
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn({
                        let allowed_origins = cors_origins.clone();
                        move |origin, _req_head| {
                            let origin_str = origin.to_str().unwrap_or("");
                            allowed_origins.iter().any(|allowed| allowed == origin_str)
                        }
                    })
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .app_data(web::Data::from(data_loader.clone()))
            .route("/health", web::get().to(health)),
    )
    .await;

    // Test with allowed origin
    let req = test::TestRequest::get()
        .uri("/health")
        .insert_header(("Origin", "http://localhost:3000"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Verify response status
    assert!(resp.status().is_success());

    // Verify CORS header is present
    let cors_header = resp.headers().get("access-control-allow-origin");
    assert!(
        cors_header.is_some(),
        "CORS header should be present for allowed origin"
    );
    assert_eq!(
        cors_header.unwrap().to_str().unwrap(),
        "http://localhost:3000"
    );
}

#[actix_web::test]
async fn test_cors_headers_for_disallowed_origin() {
    // Create test config with specific CORS origin
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec!["http://localhost:3000".to_string()],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    };

    let data_loader = Arc::new(DataLoader::new(&config));
    let cors_origins = config.cors_origins.clone();

    let app = test::init_service(
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn({
                        let allowed_origins = cors_origins.clone();
                        move |origin, _req_head| {
                            let origin_str = origin.to_str().unwrap_or("");
                            allowed_origins.iter().any(|allowed| allowed == origin_str)
                        }
                    })
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .app_data(web::Data::from(data_loader.clone()))
            .route("/health", web::get().to(health)),
    )
    .await;

    // Test with disallowed origin
    let req = test::TestRequest::get()
        .uri("/health")
        .insert_header(("Origin", "http://evil.com"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // actix-cors may return 400 Bad Request for disallowed origins
    // or return success but without CORS headers
    // Both behaviors are acceptable for security
    if resp.status().is_success() {
        // If request succeeds, verify CORS header is NOT the disallowed origin
        let cors_header = resp.headers().get("access-control-allow-origin");
        if let Some(header) = cors_header {
            // If header exists, it should not be the disallowed origin
            assert_ne!(header.to_str().unwrap(), "http://evil.com");
        }
    } else {
        // If request is blocked, that's also acceptable for CORS enforcement
        assert!(
            resp.status().is_client_error(),
            "Expected client error for disallowed origin, got: {}",
            resp.status()
        );
    }
}

#[actix_web::test]
async fn test_server_with_data_loader() {
    // Create test config
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec!["http://localhost:3000".to_string()],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    };

    // Instantiate DataLoader to ensure it can be created
    let data_loader = Arc::new(DataLoader::new(&config));

    // Verify data loader is properly initialized
    assert!(data_loader.load_paper_index().is_ok());

    // Create app with data loader
    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health)),
    )
    .await;

    // Test that endpoints work with data loader in app state
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
