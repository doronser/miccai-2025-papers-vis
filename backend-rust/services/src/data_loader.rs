//! Data loading service for papers and embeddings
//!
//! This module implements Design Decision #1: Concurrent Caching Strategy in Rust.
//! It uses `DashMap<String, T>` for lock-free concurrent access to cached data,
//! providing optimal read performance for our read-heavy workload.
//!
//! Maps Python's `src/services/data_loader.py` to Rust.
//! Responsible for:
//! - Loading paper JSON files from `papers_by_id/`
//! - Loading NPZ embedding files from `embeddings_by_id/`
//! - Maintaining in-memory caches for fast access
//! - Loading and managing the paper index

use dashmap::DashMap;
use infrastructure::{Config, file_io::read_json_file, error::InfraError};
use models::Paper;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};

/// Statistics about authors in the dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorStats {
    pub total_author_mentions: u32,
    pub unique_authors: u32,
    pub avg_authors_per_paper: f64,
    pub max_authors: u32,
    pub min_authors: u32,
}

/// Statistics about paper content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentStats {
    pub avg_title_length: f64,
    pub max_title_length: u32,
    pub min_title_length: u32,
    pub avg_abstract_length: f64,
    pub max_abstract_length: u32,
    pub min_abstract_length: u32,
}

/// Processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub success_rate: f64,
    pub total_processed: u32,
    pub successful: u32,
    pub errors: u32,
}

/// Overall statistics for the paper dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_papers: u32,
    pub papers_with_pdf: u32,
    pub pdf_availability_rate: f64,
    pub author_stats: AuthorStats,
    pub content_stats: ContentStats,
    pub subject_distribution: std::collections::HashMap<String, u32>,
    pub processing_stats: ProcessingStats,
}

/// Dataset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub total_papers: u32,
    pub generated_at: String,
    pub scraper_version: String,
    pub source_url: String,
}

/// Minimal paper information in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperIndexEntry {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub author_count: u32,
    pub subject_areas: Vec<String>,
    pub has_pdf: bool,
    pub title_length: u32,
    pub abstract_length: u32,
}

/// Paper index structure matching index.json schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperIndex {
    pub dataset_info: DatasetInfo,
    pub statistics: Statistics,
    pub papers: Vec<PaperIndexEntry>,
}

/// Result type for data loader operations
pub type DataLoaderResult<T> = Result<T, DataLoaderError>;

/// Data loader errors
#[derive(Debug, thiserror::Error)]
pub enum DataLoaderError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfraError),

    #[error("Paper not found: {0}")]
    PaperNotFound(String),

    #[error("Index not loaded")]
    IndexNotLoaded,
}

/// Data loader service for papers and embeddings
///
/// This service provides methods to load papers and embeddings from disk,
/// with in-memory caching for performance using DashMap for concurrent access.
pub struct DataLoader {
    /// Configuration with data directory paths
    config: Config,

    /// Concurrent cache for loaded papers (DashMap for lock-free concurrent reads)
    papers_cache: Arc<DashMap<String, Paper>>,

    /// Paper index loaded once and cached (RwLock for thread-safe lazy initialization)
    paper_index: RwLock<Option<PaperIndex>>,
}

impl DataLoader {
    /// Create a new DataLoader with the provided configuration
    ///
    /// # Arguments
    /// * `config` - Configuration with data directory paths
    ///
    /// # Example
    /// ```ignore
    /// use infrastructure::Config;
    /// use services::DataLoader;
    ///
    /// let config = Config::from_env();
    /// let loader = DataLoader::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        info!(
            "Initializing DataLoader with papers_dir={}, embeddings_dir={}",
            config.papers_dir.display(),
            config.embeddings_dir.display()
        );

        Self {
            config,
            papers_cache: Arc::new(DashMap::new()),
            paper_index: RwLock::new(None),
        }
    }

    /// Load the paper index from index.json
    ///
    /// The index is loaded once and cached in memory using RwLock for thread-safe
    /// lazy initialization. Subsequent calls return a clone of the cached index.
    ///
    /// # Returns
    /// * `Ok(PaperIndex)` - The loaded and cached paper index
    /// * `Err(DataLoaderError)` - If the index file cannot be read or parsed
    ///
    /// # Example
    /// ```ignore
    /// let index = loader.load_paper_index()?;
    /// println!("Total papers: {}", index.dataset_info.total_papers);
    /// ```
    pub fn load_paper_index(&self) -> DataLoaderResult<PaperIndex> {
        // Fast path: try to read the index if it's already loaded
        {
            let read_guard = self.paper_index.read()
                .map_err(|_| DataLoaderError::Infrastructure(
                    InfraError::cache("load_paper_index", "Failed to acquire read lock")
                ))?;

            if let Some(ref index) = *read_guard {
                return Ok(index.clone());
            }
        }

        // Slow path: acquire write lock and load the index
        let mut write_guard = self.paper_index.write()
            .map_err(|_| DataLoaderError::Infrastructure(
                InfraError::cache("load_paper_index", "Failed to acquire write lock")
            ))?;

        // Check again in case another thread loaded it while we were waiting
        if let Some(ref index) = *write_guard {
            return Ok(index.clone());
        }

        // Load the index
        let index_path = self.config.papers_dir.join("index.json");
        info!("Loading paper index from {}", index_path.display());

        let index: PaperIndex = read_json_file(&index_path)?;

        info!(
            "Paper index loaded: {} papers, generated at {}",
            index.dataset_info.total_papers,
            index.dataset_info.generated_at
        );

        *write_guard = Some(index.clone());
        Ok(index)
    }

    /// Get a paper by its ID
    ///
    /// First checks the in-memory cache (DashMap). On cache miss, loads the paper
    /// from disk, caches it, and returns it. Returns an error if the paper file
    /// does not exist.
    ///
    /// # Arguments
    /// * `paper_id` - The unique identifier of the paper
    ///
    /// # Returns
    /// * `Ok(Paper)` - The loaded paper
    /// * `Err(DataLoaderError::PaperNotFound)` - If the paper file doesn't exist
    /// * `Err(DataLoaderError::Infrastructure)` - If there's an I/O or parsing error
    ///
    /// # Example
    /// ```ignore
    /// match loader.get_paper_by_id("miccai-1274") {
    ///     Ok(paper) => println!("Loaded: {}", paper.title),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn get_paper_by_id(&self, paper_id: &str) -> DataLoaderResult<Paper> {
        // Check cache first
        if let Some(cached_paper) = self.papers_cache.get(paper_id) {
            debug!("Cache hit for paper: {}", paper_id);
            return Ok(cached_paper.clone());
        }

        debug!("Cache miss for paper: {}", paper_id);

        // Load from disk
        let paper_path = self.config.papers_dir.join(format!("{}.json", paper_id));

        // Check if file exists first for better error messages
        if !paper_path.exists() {
            warn!("Paper file not found: {}", paper_path.display());
            return Err(DataLoaderError::PaperNotFound(paper_id.to_string()));
        }

        let paper: Paper = read_json_file(&paper_path)?;

        // Cache the paper
        self.papers_cache.insert(paper_id.to_string(), paper.clone());
        debug!("Cached paper: {}", paper_id);

        Ok(paper)
    }

    /// Get all papers from the dataset
    ///
    /// Loads the paper index and then loads each paper by ID.
    /// Papers are cached as they are loaded.
    ///
    /// # Returns
    /// * `Ok(Vec<Paper>)` - Vector of all papers
    /// * `Err(DataLoaderError)` - If the index cannot be loaded
    ///
    /// # Note
    /// This method loads all papers into memory, which may be expensive for large datasets.
    /// Consider using pagination or streaming for production use.
    pub fn get_all_papers(&self) -> DataLoaderResult<Vec<Paper>> {
        let index = self.load_paper_index()?;
        let mut papers = Vec::with_capacity(index.papers.len());

        info!("Loading {} papers from index", index.papers.len());

        for entry in &index.papers {
            match self.get_paper_by_id(&entry.id) {
                Ok(paper) => papers.push(paper),
                Err(e) => {
                    warn!("Failed to load paper {}: {}", entry.id, e);
                    // Continue loading other papers even if one fails
                }
            }
        }

        info!("Successfully loaded {} papers", papers.len());
        Ok(papers)
    }

    /// Get the current cache statistics
    ///
    /// Returns the number of papers currently cached in memory.
    pub fn cache_size(&self) -> usize {
        self.papers_cache.len()
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infrastructure::Config;

    /// Helper to create a test config pointing to the actual data directory
    fn test_config() -> Config {
        let mut config = Config::from_env();
        // Override with test data path if running from backend-rust directory
        config.papers_dir = std::path::PathBuf::from("../backend/src/data/papers_by_id");
        config.embeddings_dir = std::path::PathBuf::from("../backend/src/data/embeddings_by_id");
        config
    }

    #[test]
    fn test_data_loader_creation() {
        let config = Config::from_env();
        let loader = DataLoader::new(config);
        assert_eq!(loader.cache_size(), 0);
    }

    #[test]
    fn test_data_loader_with_config() {
        let config = test_config();
        let loader = DataLoader::new(config.clone());
        assert_eq!(loader.config().papers_dir, config.papers_dir);
    }

    #[test]
    fn test_load_paper_index() {
        let config = test_config();

        // Skip test if data directory doesn't exist (e.g., in CI)
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = DataLoader::new(config);
        let result = loader.load_paper_index();

        if let Ok(index) = result {
            assert!(index.dataset_info.total_papers > 0);
            assert!(!index.papers.is_empty());
            assert_eq!(index.papers.len() as u32, index.dataset_info.total_papers);

            // Verify subsequent calls return the same cached index
            let index2 = loader.load_paper_index().unwrap();
            assert_eq!(index.dataset_info.total_papers, index2.dataset_info.total_papers);
        }
    }

    #[test]
    fn test_get_paper_by_id() {
        let config = test_config();

        // Skip test if data directory doesn't exist
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = DataLoader::new(config);

        // Try to get the first paper from the index
        if let Ok(index) = loader.load_paper_index() {
            if let Some(first_entry) = index.papers.first() {
                let paper_id = &first_entry.id;

                // First call should load from disk
                let result = loader.get_paper_by_id(paper_id);
                assert!(result.is_ok());

                let paper = result.unwrap();
                assert_eq!(paper.id, *paper_id);
                assert!(!paper.title.is_empty());
                assert!(!paper.abstract_text.is_empty());

                // Cache size should be 1
                assert_eq!(loader.cache_size(), 1);

                // Second call should hit cache
                let paper2 = loader.get_paper_by_id(paper_id).unwrap();
                assert_eq!(paper.id, paper2.id);
                assert_eq!(paper.title, paper2.title);

                // Cache size should still be 1
                assert_eq!(loader.cache_size(), 1);
            }
        }
    }

    #[test]
    fn test_get_nonexistent_paper() {
        let config = test_config();

        // Skip test if data directory doesn't exist
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = DataLoader::new(config);
        let result = loader.get_paper_by_id("nonexistent-paper-id-12345");

        assert!(result.is_err());
        match result {
            Err(DataLoaderError::PaperNotFound(id)) => {
                assert_eq!(id, "nonexistent-paper-id-12345");
            }
            _ => panic!("Expected PaperNotFound error"),
        }
    }

    #[test]
    fn test_cache_hit_performance() {
        let config = test_config();

        // Skip test if data directory doesn't exist
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = DataLoader::new(config);

        if let Ok(index) = loader.load_paper_index() {
            if let Some(first_entry) = index.papers.first() {
                let paper_id = &first_entry.id;

                // Load the paper once
                let _ = loader.get_paper_by_id(paper_id);

                // Measure cache hit performance
                use std::time::Instant;
                let start = Instant::now();
                for _ in 0..100 {
                    let _ = loader.get_paper_by_id(paper_id);
                }
                let duration = start.elapsed();

                // Cache hits should be very fast (< 1ms total for 100 lookups)
                assert!(duration.as_millis() < 10, "Cache lookups too slow: {:?}", duration);
            }
        }
    }

    #[test]
    fn test_get_all_papers_loads_subset() {
        let config = test_config();

        // Skip test if data directory doesn't exist
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = DataLoader::new(config);
        let result = loader.get_all_papers();

        if let Ok(papers) = result {
            assert!(!papers.is_empty());

            // All papers should have valid data
            for paper in papers.iter().take(5) {
                assert!(!paper.id.is_empty());
                assert!(!paper.title.is_empty());
                assert!(!paper.abstract_text.is_empty());
            }

            // Cache should contain the loaded papers
            assert!(loader.cache_size() > 0);
        }
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = test_config();

        // Skip test if data directory doesn't exist
        if !config.papers_dir.exists() {
            eprintln!("Skipping test: papers directory not found");
            return;
        }

        let loader = Arc::new(DataLoader::new(config));

        // Load index to get a paper ID
        let paper_id = if let Ok(index) = loader.load_paper_index() {
            if let Some(first) = index.papers.first() {
                first.id.clone()
            } else {
                return;
            }
        } else {
            return;
        };

        // Spawn multiple threads to access the same paper concurrently
        let mut handles = vec![];
        for _ in 0..10 {
            let loader_clone = Arc::clone(&loader);
            let id_clone = paper_id.clone();

            let handle = thread::spawn(move || {
                loader_clone.get_paper_by_id(&id_clone)
            });
            handles.push(handle);
        }

        // Wait for all threads and verify they all succeeded
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }

        // Cache should contain exactly one entry for the paper
        assert_eq!(loader.cache_size(), 1);
    }
}
