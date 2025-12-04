//! Health check endpoint
//!
//! Provides a simple health check endpoint for monitoring and Docker health checks.

use actix_web::{get, HttpResponse, Responder};
use tracing::info;

/// Health check endpoint
///
/// Returns a simple JSON response indicating the service is healthy.
/// This endpoint is used by Docker health checks and monitoring systems.
///
/// # Response
/// - Status: 200 OK
/// - Body: `{"status": "healthy"}`
///
/// # Example
/// ```bash
/// curl http://localhost:8000/health
/// ```
#[get("/health")]
pub async fn health() -> impl Responder {
    info!("Health check endpoint accessed");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "healthy");
    }
}
