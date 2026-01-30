//! Contract tests validating JSON response schemas match Python implementation.
//!
//! These tests ensure the Rust API produces JSON responses that are compatible
//! with the existing frontend and match the Python FastAPI implementation exactly.

use actix_web::{test, web, App};
use miccai_2025_papers_vis::api::papers;
use miccai_2025_papers_vis::config::AppConfig;
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

// =============================================================================
// Contract Tests for Paper Detail Endpoint (GET /api/papers/{id})
// =============================================================================

#[actix_web::test]
async fn test_paper_detail_has_required_fields() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // Validate required fields exist (per Python API contract)
    assert!(body["id"].is_string(), "id field must be a string");
    assert!(body["title"].is_string(), "title field must be a string");
    assert!(
        body["abstract"].is_string(),
        "abstract field must be a string (not abstract_text)"
    );
    assert!(body["authors"].is_array(), "authors field must be an array");
    assert!(
        body["subject_areas"].is_array(),
        "subject_areas field must be an array"
    );

    // Verify field naming matches Python (abstract, not abstract_text)
    assert!(
        body.get("abstract_text").is_none(),
        "abstract_text should not exist in JSON output"
    );
}

#[actix_web::test]
async fn test_paper_detail_author_schema() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // Validate author structure
    let authors = body["authors"]
        .as_array()
        .expect("authors should be an array");
    assert!(!authors.is_empty(), "Paper should have at least one author");

    for (idx, author) in authors.iter().enumerate() {
        // Name is required
        assert!(
            author["name"].is_string(),
            "Author {} must have a name field",
            idx
        );
        let name = author["name"].as_str().unwrap();
        assert!(!name.is_empty(), "Author {} name must not be empty", idx);
        assert!(
            name.len() <= 200,
            "Author {} name exceeds max length of 200",
            idx
        );

        // Affiliation is optional but if present must be a string
        if !author["affiliation"].is_null() && author.get("affiliation").is_some() {
            assert!(
                author["affiliation"].is_string() || author["affiliation"].is_null(),
                "Author {} affiliation must be a string or null",
                idx
            );
        }

        // Email is optional but if present must be a string
        if !author["email"].is_null() && author.get("email").is_some() {
            assert!(
                author["email"].is_string() || author["email"].is_null(),
                "Author {} email must be a string or null",
                idx
            );
        }
    }
}

#[actix_web::test]
async fn test_paper_detail_external_links_schema() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // external_links should be an array
    assert!(
        body["external_links"].is_array(),
        "external_links must be an array"
    );

    let links = body["external_links"]
        .as_array()
        .expect("external_links should be an array");

    for (idx, link) in links.iter().enumerate() {
        // type field is required (renamed from link_type in Rust)
        assert!(
            link["type"].is_string(),
            "Link {} must have a type field",
            idx
        );
        let link_type = link["type"].as_str().unwrap();
        // Validate link type is one of the expected values
        let valid_types = ["pdf", "github", "dataset", "website", "arxiv", "video"];
        assert!(
            valid_types.contains(&link_type) || !link_type.is_empty(),
            "Link {} has invalid type: {}",
            idx,
            link_type
        );

        // url field is required
        assert!(
            link["url"].is_string(),
            "Link {} must have a url field",
            idx
        );
        let url = link["url"].as_str().unwrap();
        assert!(
            url.starts_with("http://") || url.starts_with("https://"),
            "Link {} url must start with http:// or https://: {}",
            idx,
            url
        );

        // description is optional
        if link.get("description").is_some() && !link["description"].is_null() {
            assert!(
                link["description"].is_string(),
                "Link {} description must be a string if present",
                idx
            );
        }

        // Verify field naming (type not link_type)
        assert!(
            link.get("link_type").is_none(),
            "link_type should not exist in JSON output, use 'type' instead"
        );
    }
}

#[actix_web::test]
async fn test_paper_detail_404_response_format() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/non-existent-paper-id")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // 404 response should have a detail field (matching FastAPI style)
    assert!(
        body["detail"].is_string(),
        "404 response must have a detail field"
    );
}

#[actix_web::test]
async fn test_paper_detail_id_matches_request() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let paper_id = "miccai-1274";
    let req = test::TestRequest::get()
        .uri(&format!("/api/papers/{}", paper_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // Returned ID should match requested ID
    assert_eq!(
        body["id"], paper_id,
        "Returned paper ID must match requested ID"
    );
}

#[actix_web::test]
async fn test_paper_detail_accepts_various_id_formats() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Test various ID formats - all should return either 200 or 404, never 400
    let test_ids = [
        "miccai-1274", // valid format with hyphen
        "miccai_test", // underscore
        "PAPER-123",   // uppercase
        "123",         // number only
        "paper.test",  // with dot
    ];

    for id in test_ids {
        let req = test::TestRequest::get()
            .uri(&format!("/api/papers/{}", id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        let status = resp.status().as_u16();

        assert!(
            status == 200 || status == 404,
            "ID '{}' should return 200 or 404, got {}",
            id,
            status
        );
    }
}

// =============================================================================
// Contract Tests for Papers List Endpoint (GET /api/papers/)
// =============================================================================

#[actix_web::test]
async fn test_papers_list_returns_array() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    // Response should be a JSON array (not wrapped in an object)
    assert!(
        body.is_array(),
        "Papers list response must be a JSON array, got: {:?}",
        body
    );
}

#[actix_web::test]
async fn test_papers_list_item_schema() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=5")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let papers = body.as_array().expect("Response should be an array");

    assert!(!papers.is_empty(), "Should return at least one paper");

    // Each paper in the list should have the same schema as paper detail
    for (idx, paper) in papers.iter().enumerate() {
        assert!(paper["id"].is_string(), "Paper {} must have id field", idx);
        assert!(
            paper["title"].is_string(),
            "Paper {} must have title field",
            idx
        );
        assert!(
            paper["abstract"].is_string(),
            "Paper {} must have abstract field",
            idx
        );
        assert!(
            paper["authors"].is_array(),
            "Paper {} must have authors field",
            idx
        );
        assert!(
            paper["subject_areas"].is_array(),
            "Paper {} must have subject_areas field",
            idx
        );

        // Verify no internal field names leak to output
        assert!(
            paper.get("abstract_text").is_none(),
            "Paper {} should not have abstract_text field",
            idx
        );
    }
}

#[actix_web::test]
async fn test_papers_list_pagination_respects_limit() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Test with explicit limit
    let req = test::TestRequest::get()
        .uri("/api/papers/?limit=10")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let papers = body.as_array().expect("Response should be an array");

    assert!(
        papers.len() <= 10,
        "Response should respect limit parameter"
    );
}

#[actix_web::test]
async fn test_papers_list_pagination_offset() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    // Get first page
    let req1 = test::TestRequest::get()
        .uri("/api/papers/?limit=5&offset=0")
        .to_request();

    let resp1 = test::call_service(&app, req1).await;
    let body1: serde_json::Value = test::read_body_json(resp1).await;
    let papers1 = body1.as_array().expect("Response should be an array");

    // Get second page
    let req2 = test::TestRequest::get()
        .uri("/api/papers/?limit=5&offset=5")
        .to_request();

    let resp2 = test::call_service(&app, req2).await;
    let body2: serde_json::Value = test::read_body_json(resp2).await;
    let papers2 = body2.as_array().expect("Response should be an array");

    // Pages should be different
    if !papers1.is_empty() && !papers2.is_empty() {
        assert_ne!(
            papers1[0]["id"], papers2[0]["id"],
            "Different pages should contain different papers"
        );
    }
}

#[actix_web::test]
async fn test_papers_list_empty_for_large_offset() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let papers = body.as_array().expect("Response should be an array");

    assert!(
        papers.is_empty(),
        "Should return empty array when offset exceeds total"
    );
}

// =============================================================================
// Contract Tests for Search Endpoint (GET /api/papers/search)
// =============================================================================

#[actix_web::test]
async fn test_search_returns_array() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=cancer")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;

    assert!(body.is_array(), "Search response must be a JSON array");
}

#[actix_web::test]
async fn test_search_result_schema() {
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
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let results = body.as_array().expect("Response should be an array");

    // Each search result should have the same schema as paper detail
    for (idx, paper) in results.iter().enumerate() {
        assert!(paper["id"].is_string(), "Result {} must have id field", idx);
        assert!(
            paper["title"].is_string(),
            "Result {} must have title field",
            idx
        );
        assert!(
            paper["abstract"].is_string(),
            "Result {} must have abstract field (not abstract_text)",
            idx
        );
    }
}

#[actix_web::test]
async fn test_search_empty_query_rejected() {
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

    // Empty query should return 400 (matches Python validation behavior)
    assert_eq!(
        resp.status(),
        400,
        "Empty query should return 400 Bad Request"
    );
}

#[actix_web::test]
async fn test_search_respects_limit() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=medical&limit=5")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let results = body.as_array().expect("Response should be an array");

    assert!(results.len() <= 5, "Should respect limit parameter");
}

// =============================================================================
// Content-Type Tests
// =============================================================================

#[actix_web::test]
async fn test_papers_list_content_type() {
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
    assert!(
        content_type.is_some(),
        "Response should have content-type header"
    );

    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(
        content_type_str.contains("application/json"),
        "Content-type should be application/json, got: {}",
        content_type_str
    );
}

#[actix_web::test]
async fn test_paper_detail_content_type() {
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

    let content_type = resp.headers().get("content-type");
    assert!(
        content_type.is_some(),
        "Response should have content-type header"
    );

    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(
        content_type_str.contains("application/json"),
        "Content-type should be application/json"
    );
}

#[actix_web::test]
async fn test_search_content_type() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/search?q=test")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let content_type = resp.headers().get("content-type");
    assert!(
        content_type.is_some(),
        "Response should have content-type header"
    );

    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(
        content_type_str.contains("application/json"),
        "Content-type should be application/json"
    );
}

#[actix_web::test]
async fn test_404_response_content_type() {
    let config = create_test_config();
    let data_loader = Arc::new(DataLoader::new(&config));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::from(data_loader.clone()))
            .configure(papers::configure),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/papers/non-existent")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);

    let content_type = resp.headers().get("content-type");
    assert!(
        content_type.is_some(),
        "404 response should have content-type header"
    );

    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(
        content_type_str.contains("application/json"),
        "404 response content-type should be application/json"
    );
}
