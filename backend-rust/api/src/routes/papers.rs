use actix_web::{get, web, HttpResponse, Responder};
use models::Paper;
use services::data_loader::DataLoader;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::ApiError;

/// Query parameters for papers listing endpoint
#[derive(Debug, Deserialize)]
pub struct PapersQuery {
    /// Maximum number of papers to return (default: 50, max: 1000)
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Number of papers to skip (default: 0)
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    50
}

impl PapersQuery {
    /// Validate and sanitize query parameters
    pub fn validate(&mut self) {
        // Cap limit at 1000
        if self.limit > 1000 {
            self.limit = 1000;
        }
        // Ensure at least 1 result if limit is 0
        if self.limit == 0 {
            self.limit = 1;
        }
    }
}

/// Get all papers with pagination
///
/// Returns a paginated list of papers. Papers are loaded from the data directory
/// and cached in memory for fast subsequent access.
///
/// # Query Parameters
///
/// - `limit` (default: 50, max: 1000): Maximum number of papers to return
/// - `offset` (default: 0): Number of papers to skip
///
/// # Example
///
/// ```bash
/// # Get first 50 papers
/// curl http://localhost:8000/api/papers/
///
/// # Get 10 papers starting from offset 20
/// curl http://localhost:8000/api/papers/?limit=10&offset=20
/// ```
#[get("/api/papers/")]
pub async fn get_papers(
    query: web::Query<PapersQuery>,
    data_loader: web::Data<Arc<DataLoader>>,
) -> Result<impl Responder, ApiError> {
    log::info!(
        "Papers listing endpoint accessed (limit={}, offset={})",
        query.limit,
        query.offset
    );

    let mut query = query.into_inner();
    query.validate();

    // Load all papers
    let all_papers = data_loader.get_all_papers()?;
    let total = all_papers.len();

    log::debug!("Total papers available: {}", total);

    // Handle offset beyond total
    if query.offset >= total {
        log::debug!("Offset {} is beyond total papers {}, returning empty array", query.offset, total);
        return Ok(HttpResponse::Ok().json(Vec::<Paper>::new()));
    }

    // Apply pagination
    let end = std::cmp::min(query.offset + query.limit, total);
    let papers: Vec<Paper> = all_papers[query.offset..end].to_vec();

    log::debug!("Returning {} papers (offset={}, limit={})", papers.len(), query.offset, query.limit);

    Ok(HttpResponse::Ok().json(papers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_papers_query_default() {
        let query: PapersQuery = serde_json::from_str("{}").unwrap();
        assert_eq!(query.limit, 50);
        assert_eq!(query.offset, 0);
    }

    #[test]
    fn test_papers_query_custom() {
        let query: PapersQuery = serde_json::from_str(r#"{"limit": 100, "offset": 20}"#).unwrap();
        assert_eq!(query.limit, 100);
        assert_eq!(query.offset, 20);
    }

    #[test]
    fn test_papers_query_validate_cap_limit() {
        let mut query = PapersQuery {
            limit: 2000,
            offset: 0,
        };
        query.validate();
        assert_eq!(query.limit, 1000);
    }

    #[test]
    fn test_papers_query_validate_zero_limit() {
        let mut query = PapersQuery {
            limit: 0,
            offset: 0,
        };
        query.validate();
        assert_eq!(query.limit, 1);
    }
}
