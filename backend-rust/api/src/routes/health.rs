use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

/// Health check endpoint
///
/// Returns a simple JSON response indicating that the server is healthy.
/// This endpoint is used for health checks by Docker, Kubernetes, load balancers, etc.
///
/// # Example
///
/// ```bash
/// curl http://localhost:8000/health
/// # Returns: {"status":"healthy"}
/// ```
#[get("/health")]
pub async fn health_check() -> impl Responder {
    log::info!("Health check endpoint accessed");
    HttpResponse::Ok().json(json!({
        "status": "healthy"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "healthy");
    }
}
