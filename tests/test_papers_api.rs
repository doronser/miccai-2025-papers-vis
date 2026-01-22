use actix_web::{test, web, App};
use miccai_2025_papers_vis::api::papers;
use miccai_2025_papers_vis::config::AppConfig;
use miccai_2025_papers_vis::models::paper::Paper;
use miccai_2025_papers_vis::services::data_loader::DataLoader;
use std::sync::Arc;

fn create_test_config() -> AppConfig {
    AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8000,
        cors_origins: vec![],
        data_dir: "backend/src/data".to_string(),
        cache_dir: "target/test_cache".to_string(),
    }
}

#[actix_web::test]
async fn test_list_papers_returns_papers() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/papers/").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Expected 200 OK");

    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert!(!body.is_empty(), "Should return papers");

    // Verify all papers have required fields
    for paper in &body {
        assert!(!paper.id.is_empty());
        assert!(!paper.title.is_empty());
    }
}

#[actix_web::test]
async fn test_list_papers_pagination() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Request first page
    let req1 = test::TestRequest::get()
        .uri("/api/papers/?limit=10&offset=0")
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    assert!(resp1.status().is_success());
    let body1: Vec<Paper> = test::read_body_json(resp1).await;
    assert_eq!(body1.len(), 10);

    // Request second page
    let req2 = test::TestRequest::get()
        .uri("/api/papers/?limit=10&offset=10")
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    assert!(resp2.status().is_success());
    let body2: Vec<Paper> = test::read_body_json(resp2).await;
    assert_eq!(body2.len(), 10);

    // Verify pages are different
    assert_ne!(
        body1[0].id, body2[0].id,
        "Pages should contain different papers"
    );
}

#[actix_web::test]
async fn test_list_papers_limit_validation() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Test limit clamping to maximum (1000)
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=9999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert!(body.len() <= 1000, "Limit should be clamped to 1000");
}

#[actix_web::test]
async fn test_list_papers_offset_beyond_total() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?offset=999999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert_eq!(
        body.len(),
        0,
        "Should return empty array when offset exceeds total"
    );
}

#[actix_web::test]
async fn test_get_paper_by_id_exists() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/miccai-1274")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200, "Should return 200 for existing paper");

    let body: Paper = test::read_body_json(resp).await;
    assert_eq!(body.id, "miccai-1274");
    assert!(body.title.contains("VMRA-MaR"));
    assert!(!body.authors.is_empty());
    assert!(!body.subject_areas.is_empty());
}

#[actix_web::test]
async fn test_get_paper_by_id_not_found() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/this-paper-does-not-exist")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        404,
        "Should return 404 for non-existent paper"
    );

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["detail"], "Paper not found");
}

#[actix_web::test]
async fn test_get_paper_json_schema() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/miccai-1274")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;

    // Verify JSON schema matches Python API
    assert!(body["id"].is_string());
    assert!(body["title"].is_string());
    assert!(body["abstract"].is_string(), "abstract field should exist");
    assert!(body["authors"].is_array());
    assert!(body["subject_areas"].is_array());
    assert!(body["external_links"].is_array());

    // Verify author structure
    if let Some(authors) = body["authors"].as_array() {
        if !authors.is_empty() {
            assert!(authors[0]["name"].is_string());
        }
    }
}

#[actix_web::test]
async fn test_search_papers_finds_results() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=breast")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert!(!body.is_empty(), "Should find papers matching 'breast'");

    // Verify at least one result contains the search term
    let has_match = body.iter().any(|p| {
        p.title.to_lowercase().contains("breast")
            || p.abstract_text.to_lowercase().contains("breast")
            || p.subject_areas
                .iter()
                .any(|a| a.to_lowercase().contains("breast"))
            || p.authors
                .iter()
                .any(|a| a.name.to_lowercase().contains("breast"))
    });
    assert!(has_match, "Results should contain the search term");
}

#[actix_web::test]
async fn test_search_papers_case_insensitive() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Search with lowercase
    let req_lower = test::TestRequest::get()
        .uri("/api/papers/search?q=cancer&limit=10")
        .to_request();

    let resp_lower = test::call_service(&app, req_lower).await;
    let body_lower: Vec<Paper> = test::read_body_json(resp_lower).await;

    // Search with uppercase
    let req_upper = test::TestRequest::get()
        .uri("/api/papers/search?q=CANCER&limit=10")
        .to_request();

    let resp_upper = test::call_service(&app, req_upper).await;
    let body_upper: Vec<Paper> = test::read_body_json(resp_upper).await;

    // Should return same number of results
    assert_eq!(
        body_lower.len(),
        body_upper.len(),
        "Search should be case-insensitive"
    );
}

#[actix_web::test]
async fn test_search_papers_limit_parameter() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=cancer&limit=5")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert!(body.len() <= 5, "Should respect limit parameter");
}

#[actix_web::test]
async fn test_search_papers_empty_query_rejected() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400, "Should reject empty query");
}

#[actix_web::test]
async fn test_search_papers_default_limit() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Don't specify limit - should use default of 20
    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=medical")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<Paper> = test::read_body_json(resp).await;
    assert!(body.len() <= 20, "Should use default limit of 20");
}

#[actix_web::test]
async fn test_endpoint_response_content_type() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/papers/").to_request();

    let resp = test::call_service(&app, req).await;

    let content_type = resp.headers().get("content-type");
    assert!(content_type.is_some());

    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(
        content_type_str.contains("application/json"),
        "Should return JSON content type"
    );
}
