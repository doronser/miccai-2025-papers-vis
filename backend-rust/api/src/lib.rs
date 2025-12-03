// API crate - HTTP server, routing, CORS, error handling

pub mod errors;
pub mod routes;

// Re-export commonly used types
pub use errors::ApiError;
pub use routes::papers;
