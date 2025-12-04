//! Root endpoint
//!
//! Provides basic API information at the root path.

use actix_web::{get, HttpResponse, Responder};
use tracing::info;

/// Root endpoint
///
/// Returns API information and welcome message.
/// Matches the Python FastAPI root endpoint behavior.
///
/// # Response
/// - Status: 200 OK
/// - Body: `{"message": "MICCAI 2025 Papers Visualization API"}`
///
/// # Example
/// ```bash
/// curl http://localhost:8000/
/// ```
#[get("/")]
pub async fn root() -> impl Responder {
    info!("Root endpoint accessed");
    HttpResponse::Ok().json(serde_json::json!({
        "message": "MICCAI 2025 Papers Visualization API"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_root_endpoint() {
        let app = test::init_service(App::new().service(root)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["message"], "MICCAI 2025 Papers Visualization API");
    }
}
