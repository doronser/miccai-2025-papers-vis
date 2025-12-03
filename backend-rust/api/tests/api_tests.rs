use actix_web::{test, web, App};
use api::routes::papers;
use infrastructure::config::Config;
use services::DataLoader;
use std::env;

/// Helper function to set up test environment configuration
fn setup_test_config() -> Config {
    // Set up test environment to use the source repository data
    env::set_var(
        "DATA_DIR",
        "/l2l/src/miccai-2025-papers-vis/backend/src/data",
    );
    env::set_var("SERVER_PORT", "8000");
    env::set_var("CORS_ORIGINS", "http://localhost:3000");

    Config::from_env().expect("Failed to load test config")
}

#[actix_rt::test]
async fn test_health_check() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_rt::test]
async fn test_get_papers() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());

    let papers = body.as_array().unwrap();
    assert!(!papers.is_empty());
    assert!(papers.len() <= 50); // Default limit
}

#[actix_rt::test]
async fn test_get_papers_with_pagination() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=5&offset=0")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());

    let papers = body.as_array().unwrap();
    assert!(papers.len() <= 5);
}

#[actix_rt::test]
async fn test_get_papers_with_offset_beyond_total() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=10&offset=100000")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());

    let papers = body.as_array().unwrap();
    assert_eq!(papers.len(), 0);
}

#[actix_rt::test]
async fn test_get_paper_by_id() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    // First get a list of papers to get a valid ID
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let papers = body.as_array().unwrap();

    assert!(!papers.is_empty());
    let paper_id = papers[0]["id"].as_str().unwrap();

    // Now get that specific paper
    let req = test::TestRequest::get()
        .uri(&format!("/api/papers/{}", paper_id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], paper_id);
    assert!(body["title"].is_string());
    assert!(body["abstract"].is_string());
    assert!(body["authors"].is_array());
}

#[actix_rt::test]
async fn test_get_nonexistent_paper() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/nonexistent-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["detail"].is_string());
}

#[actix_rt::test]
async fn test_search_papers() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=medical")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());

    let results = body.as_array().unwrap();
    // Should find some results (or none if the query doesn't match)
    assert!(results.len() <= 20); // Default limit
}

#[actix_rt::test]
async fn test_search_papers_with_limit() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=a&limit=5")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.is_array());

    let results = body.as_array().unwrap();
    assert!(results.len() <= 5);
}

#[actix_rt::test]
async fn test_search_papers_empty_query() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=")
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Should return 400 Bad Request for empty query
    assert_eq!(resp.status().as_u16(), 400);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["detail"].is_string());
}

#[actix_rt::test]
async fn test_search_papers_missing_query() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search")
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Should return 400 Bad Request for missing required parameter
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_rt::test]
async fn test_search_papers_case_insensitive() {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    let app = test::init_service(
        App::new()
            .app_data(data_loader.clone())
            .configure(papers::configure_routes),
    )
    .await;

    // Search with lowercase
    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=brain&limit=5")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body1: serde_json::Value = test::read_body_json(resp).await;

    // Search with uppercase
    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=BRAIN&limit=5")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body2: serde_json::Value = test::read_body_json(resp).await;

    // Should return the same number of results (case-insensitive)
    assert_eq!(
        body1.as_array().unwrap().len(),
        body2.as_array().unwrap().len()
    );
}
