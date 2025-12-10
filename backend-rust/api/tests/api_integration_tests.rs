use actix_cors::Cors;
use actix_web::{middleware::Logger, test, web, App};
use api::routes::health::health_check;
use api::routes::papers::get_papers;
use services::data_loader::DataLoader;
use std::path::PathBuf;
use std::sync::Arc;

/// Create a test DataLoader with mock data directory
///
/// For integration tests, we'll use paths from environment variables or fallback to source data.
fn create_test_data_loader() -> Arc<DataLoader> {
    // Try multiple possible paths for test data
    let papers_dir = std::env::var("PAPERS_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let p = PathBuf::from("./backend-rust/api/tests/fixtures/papers_by_id");
            if p.exists() { Some(p) } else { None }
        })
        .or_else(|| {
            // Try from source directory (when symlink exists)
            let p = PathBuf::from("/l2l/src/miccai-2025-papers-vis/backend/src/data/papers_by_id");
            if p.exists() { Some(p) } else { None }
        })
        .unwrap_or_else(|| PathBuf::from("./data/papers_by_id"));

    let embeddings_dir = std::env::var("EMBEDDINGS_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let p = PathBuf::from("./backend-rust/api/tests/fixtures/embeddings_by_id");
            if p.exists() { Some(p) } else { None }
        })
        .or_else(|| {
            let p = PathBuf::from("/l2l/src/miccai-2025-papers-vis/backend/src/data/embeddings_by_id");
            if p.exists() { Some(p) } else { None }
        })
        .unwrap_or_else(|| PathBuf::from("./data/embeddings_by_id"));

    Arc::new(DataLoader::new(papers_dir, embeddings_dir))
}

#[actix_web::test]
async fn test_health_endpoint() {
    let data_loader = create_test_data_loader();

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_web::test]
async fn test_papers_endpoint_basic() {
    let data_loader = create_test_data_loader();

    // Skip test if data is not available
    if data_loader.load_paper_index().is_err() {
        eprintln!("Skipping test_papers_endpoint_basic: test data not available");
        return;
    }

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    let req = test::TestRequest::get().uri("/api/papers/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
}

#[actix_web::test]
async fn test_papers_endpoint_with_pagination() {
    let data_loader = create_test_data_loader();

    // Skip test if data is not available
    if data_loader.load_paper_index().is_err() {
        eprintln!("Skipping test_papers_endpoint_with_pagination: test data not available");
        return;
    }

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    // Test with limit parameter
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=5&offset=0")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
    // The actual length will depend on available test data
    // We just verify it's an array and doesn't exceed the limit
    if let Some(array) = body.as_array() {
        assert!(array.len() <= 5);
    }
}

#[actix_web::test]
async fn test_papers_endpoint_large_offset() {
    let data_loader = create_test_data_loader();

    // Skip test if data is not available
    if data_loader.load_paper_index().is_err() {
        eprintln!("Skipping test_papers_endpoint_large_offset: test data not available");
        return;
    }

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    // Test with offset beyond available data
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=10&offset=999999")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
    // Should return empty array when offset is beyond total
    if let Some(array) = body.as_array() {
        assert_eq!(array.len(), 0);
    }
}

#[actix_web::test]
async fn test_papers_endpoint_limit_cap() {
    let data_loader = create_test_data_loader();

    // Skip test if data is not available
    if data_loader.load_paper_index().is_err() {
        eprintln!("Skipping test_papers_endpoint_limit_cap: test data not available");
        return;
    }

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    // Test that limit is capped at 1000
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=5000")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());
    // Should not return more than 1000 papers (capped)
    if let Some(array) = body.as_array() {
        assert!(array.len() <= 1000);
    }
}

#[actix_web::test]
async fn test_cors_headers() {
    let data_loader = create_test_data_loader();

    // Configure CORS for this test
    let cors = Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_methods(vec!["GET"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::ACCEPT,
        ]);

    let app = test::init_service(
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(data_loader))
            .service(health_check)
            .service(get_papers)
    ).await;

    let req = test::TestRequest::get()
        .uri("/health")
        .insert_header(("Origin", "http://localhost:5173"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    // Check for CORS headers
    let headers = resp.headers();
    assert!(
        headers.contains_key("access-control-allow-origin")
            || headers.contains_key("Access-Control-Allow-Origin")
    );
}
