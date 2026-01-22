use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::models::paper::Paper;
use crate::services::data_loader::DataLoader;

/// Query parameters for listing papers with pagination
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

/// Query parameters for searching papers
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_search_limit")]
    pub limit: usize,
}

fn default_search_limit() -> usize {
    20
}

/// Handler for GET /api/papers/ - List papers with pagination
///
/// Accepts query parameters:
/// - limit: number of papers to return (default 50, range 1-1000)
/// - offset: number of papers to skip (default 0, min 0)
///
/// Returns: JSON array of Paper objects
pub async fn list_papers(
    query: web::Query<ListQuery>,
    data_loader: web::Data<DataLoader>,
) -> Result<HttpResponse> {
    // Validate limit range
    let limit = query.limit.clamp(1, 1000);
    let offset = query.offset;

    // Get all papers
    let all_papers = data_loader.get_all_papers().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to load papers: {}", e))
    })?;

    let total = all_papers.len();

    // Return empty array if offset is beyond total
    if offset >= total {
        return Ok(HttpResponse::Ok().json(Vec::<Paper>::new()));
    }

    // Calculate end index
    let end = (offset + limit).min(total);

    // Return paginated slice - need to clone for owned Vec
    let papers: Vec<Paper> = all_papers[offset..end].to_vec();
    Ok(HttpResponse::Ok().json(papers))
}

/// Handler for GET /api/papers/{id} - Get a specific paper by ID
///
/// Path parameter:
/// - id: paper ID (e.g., "miccai-1274")
///
/// Returns:
/// - 200 with Paper JSON if found
/// - 404 if paper not found
pub async fn get_paper(
    paper_id: web::Path<String>,
    data_loader: web::Data<DataLoader>,
) -> Result<HttpResponse> {
    let id = paper_id.into_inner();

    match data_loader.get_paper_by_id(&id) {
        Ok(Some(paper)) => Ok(HttpResponse::Ok().json(paper)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "detail": "Paper not found"
        }))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to load paper: {}",
            e
        ))),
    }
}

/// Handler for GET /api/papers/search - Search papers by query
///
/// Query parameters:
/// - q: query string (min length 1)
/// - limit: number of results to return (default 20, range 1-100)
///
/// Returns: JSON array of Paper objects matching the search criteria
pub async fn search_papers(
    query: web::Query<SearchQuery>,
    data_loader: web::Data<DataLoader>,
) -> Result<HttpResponse> {
    // Validate query
    if query.q.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "detail": "Query parameter 'q' must not be empty"
        })));
    }

    // Validate limit range
    let limit = query.limit.clamp(1, 100);

    // Perform search
    let results = data_loader
        .search_papers(&query.q, limit)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Search failed: {}", e)))?;

    Ok(HttpResponse::Ok().json(results))
}

/// Configure paper routes under /api/papers scope
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/papers")
            .route("/", web::get().to(list_papers))
            .route("/search", web::get().to(search_papers))
            .route("/{id}", web::get().to(get_paper)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use actix_web::{test, App};

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

    #[actix_web::test]
    async fn test_list_papers_default() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/papers/").to_request();

        let resp = test::call_service(&app, req).await;
        let status = resp.status();
        if !status.is_success() {
            let body = test::read_body(resp).await;
            let body_str = String::from_utf8_lossy(&body);
            panic!("Error status: {}, body: {}", status, body_str);
        }

        let body: Vec<Paper> = test::read_body_json(resp).await;
        assert!(!body.is_empty());
        assert!(body.len() <= 50); // Default limit
    }

    #[actix_web::test]
    async fn test_list_papers_with_pagination() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/?limit=10&offset=5")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<Paper> = test::read_body_json(resp).await;
        assert!(body.len() <= 10);
    }

    #[actix_web::test]
    async fn test_list_papers_offset_beyond_total() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/?offset=99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<Paper> = test::read_body_json(resp).await;
        assert_eq!(body.len(), 0);
    }

    #[actix_web::test]
    async fn test_get_paper_by_id_success() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/miccai-1274")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Paper = test::read_body_json(resp).await;
        assert_eq!(body.id, "miccai-1274");
        assert!(body.title.contains("VMRA-MaR"));
    }

    #[actix_web::test]
    async fn test_get_paper_by_id_not_found() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/nonexistent-id")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_search_papers_success() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/search?q=breast")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<Paper> = test::read_body_json(resp).await;
        assert!(!body.is_empty());

        // Verify results contain the search term
        let has_match = body.iter().any(|p| {
            p.title.to_lowercase().contains("breast")
                || p.abstract_text.to_lowercase().contains("breast")
                || p.subject_areas
                    .iter()
                    .any(|a| a.to_lowercase().contains("breast"))
        });
        assert!(has_match);
    }

    #[actix_web::test]
    async fn test_search_papers_with_limit() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/search?q=cancer&limit=5")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Vec<Paper> = test::read_body_json(resp).await;
        assert!(body.len() <= 5);
    }

    #[actix_web::test]
    async fn test_search_papers_empty_query() {
        let config = create_test_config();
        let data_loader = Arc::new(DataLoader::new(&config));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(data_loader.clone()))
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/papers/search?q=")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }
}
