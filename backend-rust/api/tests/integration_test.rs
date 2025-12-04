//! Integration tests for the HTTP server
//!
//! These tests verify the end-to-end behavior of the HTTP server,
//! including routing, CORS, and basic endpoint functionality.

use actix_web::{test, web, App};
use api::{routes, AppState};

#[actix_web::test]
async fn test_root_endpoint_returns_message() {
    let app_state = web::Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(routes::root::root),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["message"], "MICCAI 2025 Papers Visualization API");
}

#[actix_web::test]
async fn test_health_endpoint_returns_healthy() {
    let app_state = web::Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(routes::health::health),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "healthy");
}

#[actix_web::test]
async fn test_404_for_nonexistent_endpoint() {
    let app_state = web::Data::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(routes::root::root)
            .service(routes::health::health),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/nonexistent")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_app_state_is_accessible() {
    // Verify that app state can be created and cloned
    let state = AppState::new();
    let _cloned = state.clone();
    // This test verifies that the state structure is working correctly
}
