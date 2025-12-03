use crate::ServiceError;
use dashmap::DashMap;
use infrastructure::config::Config;
use infrastructure::file_io::{read_json_file, read_npz_file};
use models::Paper;
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Paper index metadata from index.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub total_papers: u32,
    pub generated_at: String,
    pub scraper_version: String,
    pub source_url: String,
}

/// Paper reference in the index (contains just the ID and title)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperRef {
    pub id: String,
    pub title: String,
}

/// Main paper index structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperIndex {
    pub dataset_info: DatasetInfo,
    pub statistics: serde_json::Value,
    pub papers: Vec<PaperRef>,
}

/// DataLoader service for loading papers and embeddings with caching
///
/// This service provides cached access to paper data and embeddings from the file system.
/// It uses DashMap for lock-free concurrent access to cached data, implementing
/// Design Decision #4 (Concurrent Data Access and Caching Strategy).
///
/// Papers and embeddings are lazily loaded on first access and then cached,
/// implementing Design Decision #2 (NPZ Embedding File Reading Strategy).
pub struct DataLoader {
    config: Arc<Config>,
    papers_cache: Arc<DashMap<String, Paper>>,
    embeddings_cache: Arc<DashMap<String, Array1<f32>>>,
    paper_index: Arc<RwLock<Option<PaperIndex>>>,
}

impl DataLoader {
    /// Create a new DataLoader with the given configuration
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            papers_cache: Arc::new(DashMap::new()),
            embeddings_cache: Arc::new(DashMap::new()),
            paper_index: Arc::new(RwLock::new(None)),
        }
    }

    /// Load the paper index from disk
    ///
    /// The index is cached in memory after the first load.
    /// Returns a clone of the cached index.
    pub fn load_paper_index(&self) -> Result<PaperIndex, ServiceError> {
        // Check if already cached (read lock)
        {
            let index_guard = self.paper_index.read().unwrap();
            if let Some(ref index) = *index_guard {
                return Ok(index.clone());
            }
        }

        // Not cached, load from disk (write lock)
        let mut index_guard = self.paper_index.write().unwrap();

        // Double-check in case another thread loaded it while we waited for the write lock
        if let Some(ref index) = *index_guard {
            return Ok(index.clone());
        }

        let index_path = self.config.papers_dir.join("index.json");
        let index: PaperIndex = read_json_file(&index_path)?;

        // Cache the index
        *index_guard = Some(index.clone());

        Ok(index)
    }

    /// Get a paper by ID
    ///
    /// Checks the cache first, then loads from disk if not cached.
    /// Returns None if the paper doesn't exist.
    pub fn get_paper_by_id(&self, paper_id: &str) -> Result<Option<Paper>, ServiceError> {
        // Check cache first
        if let Some(paper_ref) = self.papers_cache.get(paper_id) {
            return Ok(Some(paper_ref.value().clone()));
        }

        // Not in cache, try to load from disk
        let paper_path = self.config.papers_dir.join(format!("{}.json", paper_id));

        // Check if file exists
        if !paper_path.exists() {
            return Ok(None);
        }

        // Load from disk
        let paper: Paper = read_json_file(&paper_path)?;

        // Cache it
        self.papers_cache.insert(paper_id.to_string(), paper.clone());

        Ok(Some(paper))
    }

    /// Get all papers
    ///
    /// Loads papers from the index and returns them all.
    /// Uses caching for individual papers.
    pub fn get_all_papers(&self) -> Result<Vec<Paper>, ServiceError> {
        let index = self.load_paper_index()?;
        let mut papers = Vec::with_capacity(index.papers.len());

        for paper_ref in index.papers.iter() {
            if let Some(paper) = self.get_paper_by_id(&paper_ref.id)? {
                papers.push(paper);
            }
        }

        Ok(papers)
    }

    /// Get an embedding by paper ID
    ///
    /// Checks the cache first, then loads from disk if not cached.
    /// Returns None if the embedding doesn't exist.
    pub fn get_embedding_by_id(&self, paper_id: &str) -> Result<Option<Array1<f32>>, ServiceError> {
        // Check cache first
        if let Some(embedding_ref) = self.embeddings_cache.get(paper_id) {
            return Ok(Some(embedding_ref.value().clone()));
        }

        // Not in cache, try to load from disk
        let embedding_path = self
            .config
            .embeddings_dir
            .join(format!("{}_embedding.npz", paper_id));

        // Check if file exists
        if !embedding_path.exists() {
            return Ok(None);
        }

        // Load from disk
        let npz_data = read_npz_file(&embedding_path)?;

        // Cache it
        self.embeddings_cache
            .insert(paper_id.to_string(), npz_data.embedding.clone());

        Ok(Some(npz_data.embedding))
    }

    /// Get all embeddings
    ///
    /// Loads embeddings for all papers in the index.
    /// Uses lazy loading and caching for individual embeddings.
    pub fn get_all_embeddings(&self) -> Result<HashMap<String, Array1<f32>>, ServiceError> {
        let index = self.load_paper_index()?;
        let mut embeddings = HashMap::with_capacity(index.papers.len());

        for paper_ref in index.papers.iter() {
            if let Some(embedding) = self.get_embedding_by_id(&paper_ref.id)? {
                embeddings.insert(paper_ref.id.clone(), embedding);
            }
        }

        Ok(embeddings)
    }

    /// Search papers by text query
    ///
    /// Searches in title, abstract, subject areas, and author names.
    /// Returns up to `limit` matching papers.
    ///
    /// If the query is empty, returns an empty vector.
    pub fn search_papers(&self, query: &str, limit: usize) -> Result<Vec<Paper>, ServiceError> {
        // Empty query returns no results
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let all_papers = self.get_all_papers()?;
        let query_lower = query.to_lowercase();

        let mut matching_papers = Vec::new();

        for paper in all_papers {
            // Check if query matches in any field
            let matches = query_lower.is_empty()
                || paper.title.to_lowercase().contains(&query_lower)
                || paper.abstract_text.to_lowercase().contains(&query_lower)
                || paper
                    .subject_areas
                    .iter()
                    .any(|area| area.to_lowercase().contains(&query_lower))
                || paper
                    .authors
                    .iter()
                    .any(|author| author.name.to_lowercase().contains(&query_lower));

            if matches {
                matching_papers.push(paper);

                // Stop if we've reached the limit
                if matching_papers.len() >= limit {
                    break;
                }
            }
        }

        Ok(matching_papers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infrastructure::config::Config;
    use serial_test::serial;
    use std::env;

    fn get_test_config() -> Config {
        // Set up test environment to use the source repository data
        env::set_var("DATA_DIR", "/l2l/src/miccai-2025-papers-vis/backend/src/data");
        Config::from_env().unwrap()
    }

    #[test]
    #[serial]
    #[serial]
    fn test_data_loader_initialization() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Verify caches are empty
        assert_eq!(loader.papers_cache.len(), 0);
        assert_eq!(loader.embeddings_cache.len(), 0);
    }

    #[test]
    #[serial]
    fn test_load_paper_index() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let result = loader.load_paper_index();
        assert!(result.is_ok());

        let index = result.unwrap();
        assert_eq!(index.dataset_info.total_papers, 1008);
        assert!(!index.papers.is_empty());
    }

    #[test]
    #[serial]
    fn test_load_paper_index_caching() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Load once
        let result1 = loader.load_paper_index();
        assert!(result1.is_ok());

        // Load again - should be cached
        let result2 = loader.load_paper_index();
        assert!(result2.is_ok());

        // Both should be equal
        let index1 = result1.unwrap();
        let index2 = result2.unwrap();
        assert_eq!(index1.dataset_info.total_papers, index2.dataset_info.total_papers);
        assert_eq!(index1.papers.len(), index2.papers.len());
    }

    #[test]
    #[serial]
    fn test_get_paper_by_id() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Load index to get a valid paper ID
        let index = loader.load_paper_index().unwrap();
        assert!(!index.papers.is_empty());

        let paper_id = &index.papers[0].id;

        // Get the paper
        let result = loader.get_paper_by_id(paper_id);
        assert!(result.is_ok());

        let paper = result.unwrap();
        assert!(paper.is_some());

        let paper = paper.unwrap();
        assert_eq!(paper.id, *paper_id);
        assert!(!paper.title.is_empty());
        assert!(!paper.abstract_text.is_empty());
        assert!(!paper.authors.is_empty());
    }

    #[test]
    #[serial]
    fn test_get_paper_by_id_caching() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let index = loader.load_paper_index().unwrap();
        let paper_id = &index.papers[0].id;

        // Get the paper - should load from disk
        let paper1 = loader.get_paper_by_id(paper_id).unwrap().unwrap();

        // Cache should now contain the paper
        assert_eq!(loader.papers_cache.len(), 1);

        // Get again - should come from cache
        let paper2 = loader.get_paper_by_id(paper_id).unwrap().unwrap();

        assert_eq!(paper1.id, paper2.id);
        assert_eq!(paper1.title, paper2.title);
    }

    #[test]
    #[serial]
    fn test_get_nonexistent_paper() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let result = loader.get_paper_by_id("nonexistent-id");
        assert!(result.is_ok());

        let paper = result.unwrap();
        assert!(paper.is_none());
    }

    #[test]
    #[serial]
    fn test_get_all_papers() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let result = loader.get_all_papers();
        assert!(result.is_ok());

        let papers = result.unwrap();
        assert!(!papers.is_empty());
        assert_eq!(papers.len(), 1008);

        // Verify structure of first paper
        let paper = &papers[0];
        assert!(!paper.id.is_empty());
        assert!(!paper.title.is_empty());
        assert!(!paper.abstract_text.is_empty());
    }

    #[test]
    #[serial]
    fn test_get_embedding_by_id() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Load index to get a valid paper ID
        let index = loader.load_paper_index().unwrap();
        let paper_id = &index.papers[0].id;

        // Try to get the embedding
        let result = loader.get_embedding_by_id(paper_id);

        // Embedding might not exist for all papers, so we handle both cases
        if result.is_ok() {
            if let Some(embedding) = result.unwrap() {
                // Should be 768-dimensional (SciBERT)
                assert_eq!(embedding.len(), 768);
            }
        }
    }

    #[test]
    #[serial]
    fn test_get_embedding_by_id_caching() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let index = loader.load_paper_index().unwrap();
        let paper_id = &index.papers[0].id;

        // Get the embedding - should load from disk
        let embedding1_opt = loader.get_embedding_by_id(paper_id).unwrap();

        if let Some(embedding1) = embedding1_opt {
            // Cache should now contain the embedding
            assert!(loader.embeddings_cache.len() >= 1);

            // Get again - should come from cache
            let embedding2 = loader.get_embedding_by_id(paper_id).unwrap().unwrap();

            assert_eq!(embedding1.len(), embedding2.len());
            assert_eq!(embedding1, embedding2);
        }
    }

    #[test]
    #[serial]
    fn test_get_all_embeddings() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let result = loader.get_all_embeddings();
        assert!(result.is_ok());

        let embeddings = result.unwrap();
        // Should have embeddings for most papers
        assert!(!embeddings.is_empty());

        // Check that embeddings are 768-dimensional
        for (_paper_id, embedding) in embeddings.iter().take(5) {
            assert_eq!(embedding.len(), 768);
        }
    }

    #[test]
    #[serial]
    fn test_search_papers() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let results = loader.search_papers("medical", 10).unwrap();
        assert!(!results.is_empty());
        assert!(results.len() <= 10);

        // Verify that results match the query
        for paper in results {
            let query_lower = "medical";
            let found_match = paper.title.to_lowercase().contains(query_lower)
                || paper.abstract_text.to_lowercase().contains(query_lower)
                || paper
                    .subject_areas
                    .iter()
                    .any(|area| area.to_lowercase().contains(query_lower))
                || paper
                    .authors
                    .iter()
                    .any(|author| author.name.to_lowercase().contains(query_lower));
            assert!(found_match);
        }
    }

    #[test]
    #[serial]
    fn test_search_papers_empty_query() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let results = loader.search_papers("", 10).unwrap();
        // Empty query should return empty results
        assert_eq!(results.len(), 0);
    }

    #[test]
    #[serial]
    fn test_search_papers_limit() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Search for a common term with a small limit
        let results = loader.search_papers("a", 5).unwrap();
        assert!(results.len() <= 5);
    }

    #[test]
    #[serial]
    fn test_search_papers_in_title() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Search for something likely in titles
        let results = loader.search_papers("learning", 10).unwrap();
        assert!(!results.is_empty());

        // At least one should have "learning" in the title
        let has_match = results
            .iter()
            .any(|p| p.title.to_lowercase().contains("learning"));
        assert!(has_match);
    }

    #[test]
    #[serial]
    fn test_search_papers_in_authors() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        // Get a paper first to extract an author name
        let index = loader.load_paper_index().unwrap();
        let paper_id = &index.papers[0].id;
        let paper = loader.get_paper_by_id(paper_id).unwrap().unwrap();

        if !paper.authors.is_empty() {
            let author_name = &paper.authors[0].name;
            // Search by first word of author name
            let search_term = author_name.split_whitespace().next().unwrap_or("");
            if !search_term.is_empty() {
                let results = loader.search_papers(search_term, 10).unwrap();
                // Should find at least the paper we got the author from
                assert!(!results.is_empty());
            }
        }
    }

    #[test]
    #[serial]
    fn test_search_papers_case_insensitive() {
        let config = get_test_config();
        let loader = DataLoader::new(config);

        let results_lower = loader.search_papers("brain", 5).unwrap();
        let results_upper = loader.search_papers("BRAIN", 5).unwrap();
        let results_mixed = loader.search_papers("Brain", 5).unwrap();

        // Should all return the same results (case-insensitive)
        assert_eq!(results_lower.len(), results_upper.len());
        assert_eq!(results_lower.len(), results_mixed.len());
    }
}
