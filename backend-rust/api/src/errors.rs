use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

/// API layer error types that map to HTTP responses
///
/// This enum implements Design Decision #3 (Error Handling and HTTP Response Strategy)
/// by providing a unified error type that converts service layer errors to appropriate
/// HTTP status codes and JSON error responses.
#[derive(Debug)]
pub enum ApiError {
    /// Resource not found (404)
    NotFound(String),

    /// Bad request / validation error (400)
    BadRequest(String),

    /// Internal server error (500)
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status_code).json(json!({
            "detail": error_message
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Convert ServiceError to ApiError
impl From<services::ServiceError> for ApiError {
    fn from(err: services::ServiceError) -> Self {
        match err {
            services::ServiceError::PaperNotFound(msg) => ApiError::NotFound(msg),
            services::ServiceError::EmbeddingNotFound(msg) => ApiError::NotFound(msg),
            services::ServiceError::InfrastructureError(_) => {
                ApiError::InternalServerError(err.to_string())
            }
        }
    }
}
