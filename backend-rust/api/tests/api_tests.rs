use actix_web::test;

mod helpers;
use helpers::{assert_json_response, assert_status, create_test_app};

// ============================================================================
// Basic Endpoint Tests (from test_api.py lines 8-68)
// ============================================================================

#[actix_rt::test]
async fn test_root_endpoint() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    assert!(body["message"].is_string());
    assert_eq!(body["message"], "MICCAI 2025 Papers Visualization API");
}

#[actix_rt::test]
async fn test_health_endpoint() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_rt::test]
async fn test_papers_endpoint() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    assert!(body.is_array());

    let papers = body.as_array().unwrap();
    assert!(!papers.is_empty(), "Should have some papers loaded");
}

#[actix_rt::test]
async fn test_papers_pagination() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=5&offset=0")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    let papers = body.as_array().unwrap();
    assert!(papers.len() <= 5);
}

#[actix_rt::test]
async fn test_search_papers() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=medical")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    assert!(body.is_array());
}

#[actix_rt::test]
async fn test_search_papers_empty_query() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=")
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Should return 422 validation error (matching Python FastAPI behavior)
    // Note: In Rust we return 400 Bad Request, which is acceptable for validation errors
    assert_status(&resp, 400);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["detail"].is_string());
}

#[actix_rt::test]
async fn test_get_paper_by_id() {
    let app = test::init_service(create_test_app()).await;

    // First get a paper ID from the papers list
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let papers = body.as_array().unwrap();

    if !papers.is_empty() {
        let paper_id = papers[0]["id"].as_str().unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/api/papers/{}", paper_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        let body: serde_json::Value = assert_json_response(resp, 200).await;
        assert_eq!(body["id"], paper_id);
        assert!(body["title"].is_string());
        assert!(body["abstract"].is_string());
        assert!(body["authors"].is_array());
    }
}

#[actix_rt::test]
async fn test_get_nonexistent_paper() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/nonexistent-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_status(&resp, 404);
}

// ============================================================================
// Additional Tests for Complete Coverage
// ============================================================================

#[actix_rt::test]
async fn test_get_papers_with_offset_beyond_total() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=10&offset=100000")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    let papers = body.as_array().unwrap();
    assert_eq!(papers.len(), 0);
}

#[actix_rt::test]
async fn test_search_papers_with_limit() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=a&limit=5")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let body: serde_json::Value = assert_json_response(resp, 200).await;
    let results = body.as_array().unwrap();
    assert!(results.len() <= 5);
}

#[actix_rt::test]
async fn test_search_papers_missing_query() {
    let app = test::init_service(create_test_app()).await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search")
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Should return 400 Bad Request for missing required parameter
    assert_status(&resp, 400);
}

#[actix_rt::test]
async fn test_search_papers_case_insensitive() {
    let app = test::init_service(create_test_app()).await;

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
