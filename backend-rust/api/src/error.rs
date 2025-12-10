use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use services::data_loader::DataLoaderError;
use std::fmt;

/// Custom error type for API operations that implements actix-web's ResponseError trait
///
/// This error type maps service-layer errors to appropriate HTTP status codes and
/// provides meaningful error messages to API clients.
#[derive(Debug)]
#[allow(dead_code)] // Some variants will be used in future milestones
pub enum ApiError {
    /// Data loading error (500 Internal Server Error)
    DataLoaderError(DataLoaderError),
    /// Resource not found (404 Not Found)
    NotFound(String),
    /// Invalid request parameters (400 Bad Request)
    BadRequest(String),
    /// Internal server error (500 Internal Server Error)
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DataLoaderError(e) => write!(f, "Data loading error: {}", e),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DataLoaderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();

        HttpResponse::build(status_code).json(serde_json::json!({
            "error": error_message,
            "status": status_code.as_u16()
        }))
    }
}

impl From<DataLoaderError> for ApiError {
    fn from(error: DataLoaderError) -> Self {
        ApiError::DataLoaderError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let error = ApiError::NotFound("Paper not found".to_string());
        assert_eq!(error.to_string(), "Not found: Paper not found");
    }

    #[test]
    fn test_api_error_status_codes() {
        assert_eq!(
            ApiError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::BadRequest("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::InternalError("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
