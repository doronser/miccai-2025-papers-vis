use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use services::DataLoader;

use crate::errors::ApiError;

/// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

/// Query parameters for search
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_search_limit")]
    pub limit: usize,
}

fn default_search_limit() -> usize {
    20
}

/// GET /api/papers/
///
/// Get all papers with pagination.
/// Query parameters:
/// - limit: Number of papers to return (default: 50, max: 1000)
/// - offset: Number of papers to skip (default: 0)
pub async fn get_papers(
    data: web::Data<DataLoader>,
    query: web::Query<PaginationQuery>,
) -> Result<impl Responder, ApiError> {
    info!(
        "GET /api/papers/ - limit: {}, offset: {}",
        query.limit, query.offset
    );

    // Validate limit
    let limit = query.limit.min(1000).max(1);

    // Get all papers
    let all_papers = data.get_all_papers().map_err(ApiError::from)?;
    let total = all_papers.len();

    // Handle offset out of bounds
    if query.offset >= total {
        return Ok(HttpResponse::Ok().json(Vec::<models::Paper>::new()));
    }

    // Slice the papers
    let end = (query.offset + limit).min(total);
    let papers_slice = &all_papers[query.offset..end];

    Ok(HttpResponse::Ok().json(papers_slice))
}

/// GET /api/papers/{id}
///
/// Get a specific paper by ID.
/// Returns 404 if the paper is not found.
pub async fn get_paper_by_id(
    data: web::Data<DataLoader>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let paper_id = path.into_inner();
    info!("GET /api/papers/{}", paper_id);

    match data.get_paper_by_id(&paper_id).map_err(ApiError::from)? {
        Some(paper) => Ok(HttpResponse::Ok().json(paper)),
        None => Err(ApiError::NotFound("Paper not found".to_string())),
    }
}

/// GET /api/papers/search
///
/// Search papers by text query.
/// Query parameters:
/// - q: Search query (required, min length: 1)
/// - limit: Number of results to return (default: 20, max: 100)
pub async fn search_papers(
    data: web::Data<DataLoader>,
    query: web::Query<SearchQuery>,
) -> Result<impl Responder, ApiError> {
    info!("GET /api/papers/search?q={}", query.q);

    // Validate query
    if query.q.is_empty() {
        return Err(ApiError::BadRequest(
            "Query parameter 'q' must not be empty".to_string(),
        ));
    }

    // Validate and cap limit
    let limit = query.limit.min(100).max(1);

    // Search papers
    let results = data
        .search_papers(&query.q, limit)
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().json(results))
}

/// GET /health
///
/// Health check endpoint.
pub async fn health_check() -> impl Responder {
    info!("GET /health");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

/// GET /
///
/// Root endpoint - API information.
pub async fn root() -> impl Responder {
    info!("GET /");
    HttpResponse::Ok().json(serde_json::json!({
        "message": "MICCAI 2025 Papers Visualization API"
    }))
}

/// Configure paper routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/papers")
            // Register search route before {id} to avoid path conflicts
            .route("/search", web::get().to(search_papers))
            .route("/", web::get().to(get_papers))
            .route("/{id}", web::get().to(get_paper_by_id)),
    )
    .route("/", web::get().to(root))
    .route("/health", web::get().to(health_check));
}
