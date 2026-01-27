use crate::config::AppConfig;
use crate::models::paper::Paper;
use crate::npy_parser;
use anyhow::{Context, Result};
use ndarray::Array1;
use parking_lot::RwLock;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

/// Index metadata structure matching the Python index.json format
#[derive(Debug, Deserialize)]
struct PaperIndex {
    papers: Vec<PaperIndexEntry>,
}

/// Individual paper entry in the index
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct PaperIndexEntry {
    id: String,
    title: String,
    authors: Vec<String>,
    #[serde(default)]
    author_count: usize,
    #[serde(default)]
    subject_areas: Vec<String>,
    #[serde(default)]
    has_pdf: bool,
    #[serde(default)]
    title_length: usize,
    #[serde(default)]
    abstract_length: usize,
}

/// DataLoader service for loading papers and embeddings from disk with caching
pub struct DataLoader {
    #[allow(dead_code)]
    data_dir: PathBuf,
    papers_dir: PathBuf,
    embeddings_dir: PathBuf,
    papers_cache: Arc<RwLock<HashMap<String, Paper>>>,
    embeddings_cache: Arc<RwLock<HashMap<String, Array1<f32>>>>,
    paper_index: Arc<RwLock<Option<PaperIndex>>>,
}

impl DataLoader {
    /// Create a new DataLoader instance
    pub fn new(config: &AppConfig) -> Self {
        let data_dir = PathBuf::from(&config.data_dir);
        let papers_dir = data_dir.join("papers_by_id");
        let embeddings_dir = data_dir.join("embeddings_by_id");

        Self {
            data_dir,
            papers_dir,
            embeddings_dir,
            papers_cache: Arc::new(RwLock::new(HashMap::new())),
            embeddings_cache: Arc::new(RwLock::new(HashMap::new())),
            paper_index: Arc::new(RwLock::new(None)),
        }
    }

    /// Load the main paper index with all paper IDs and metadata
    pub fn load_paper_index(&self) -> Result<()> {
        // Check if already loaded
        {
            let index = self.paper_index.read();
            if index.is_some() {
                return Ok(());
            }
        }

        // Load the index
        let index_path = self.papers_dir.join("index.json");
        let file = File::open(&index_path)
            .with_context(|| format!("Failed to open index file: {:?}", index_path))?;

        let index: PaperIndex = serde_json::from_reader(file)
            .with_context(|| format!("Failed to parse index file: {:?}", index_path))?;

        // Store in cache
        let mut index_lock = self.paper_index.write();
        *index_lock = Some(index);

        Ok(())
    }

    /// Get a paper by its ID
    pub fn get_paper_by_id(&self, paper_id: &str) -> Result<Option<Paper>> {
        // Check cache first
        {
            let cache = self.papers_cache.read();
            if let Some(paper) = cache.get(paper_id) {
                return Ok(Some(paper.clone()));
            }
        }

        // Try to load from disk
        let paper_path = self.papers_dir.join(format!("{}.json", paper_id));

        if !paper_path.exists() {
            return Ok(None);
        }

        let file = File::open(&paper_path)
            .with_context(|| format!("Failed to open paper file: {:?}", paper_path))?;

        let paper: Paper = serde_json::from_reader(file)
            .with_context(|| format!("Failed to parse paper file: {:?}", paper_path))?;

        // Store in cache
        let mut cache = self.papers_cache.write();
        cache.insert(paper_id.to_string(), paper.clone());

        Ok(Some(paper))
    }

    /// Get all papers from the index
    pub fn get_all_papers(&self) -> Result<Vec<Paper>> {
        // Ensure index is loaded
        self.load_paper_index()?;

        let index = self.paper_index.read();
        let index = index.as_ref().expect("Index should be loaded");

        let mut papers = Vec::new();
        for entry in &index.papers {
            if let Some(paper) = self.get_paper_by_id(&entry.id)? {
                papers.push(paper);
            }
        }

        Ok(papers)
    }

    /// Load an embedding from an NPZ file
    pub fn get_embedding_by_id(&self, paper_id: &str) -> Result<Option<Array1<f32>>> {
        // Check cache first
        {
            let cache = self.embeddings_cache.read();
            if let Some(embedding) = cache.get(paper_id) {
                return Ok(Some(embedding.clone()));
            }
        }

        // Try to load from disk
        let embedding_path = self
            .embeddings_dir
            .join(format!("{}_embedding.npz", paper_id));

        if !embedding_path.exists() {
            return Ok(None);
        }

        // NPZ files are ZIP archives containing NPY files
        // We need to extract and read the NPY data
        let file = File::open(&embedding_path)
            .with_context(|| format!("Failed to open embedding file: {:?}", embedding_path))?;

        let mut archive = zip::ZipArchive::new(file)
            .with_context(|| format!("Failed to read NPZ archive: {:?}", embedding_path))?;

        // Look for the 'embedding.npy' file in the archive
        let mut npy_file = archive.by_name("embedding.npy").with_context(|| {
            format!(
                "Failed to find embedding.npy in archive: {:?}",
                embedding_path
            )
        })?;

        // Read the NPY file content
        let mut buffer = Vec::new();
        npy_file.read_to_end(&mut buffer).with_context(|| {
            format!("Failed to read NPY data from archive: {:?}", embedding_path)
        })?;

        // Parse the NPY format manually
        let embedding = npy_parser::parse_npy_f32(&buffer)
            .with_context(|| format!("Failed to parse NPY data: {:?}", embedding_path))?;

        // Store in cache
        let mut cache = self.embeddings_cache.write();
        cache.insert(paper_id.to_string(), embedding.clone());

        Ok(Some(embedding))
    }

    /// Get all embeddings for papers in the index
    pub fn get_all_embeddings(&self) -> Result<HashMap<String, Array1<f32>>> {
        // Ensure index is loaded
        self.load_paper_index()?;

        let index = self.paper_index.read();
        let index = index.as_ref().expect("Index should be loaded");

        let mut embeddings = HashMap::new();
        for entry in &index.papers {
            if let Some(embedding) = self.get_embedding_by_id(&entry.id)? {
                embeddings.insert(entry.id.clone(), embedding);
            }
        }

        Ok(embeddings)
    }

    /// Search papers by query string (case-insensitive)
    /// Searches in title, abstract, subject areas, and author names
    pub fn search_papers(&self, query: &str, limit: usize) -> Result<Vec<Paper>> {
        let all_papers = self.get_all_papers()?;
        let query_lower = query.to_lowercase();

        let mut matching_papers = Vec::new();

        for paper in all_papers {
            // Check title
            if paper.title.to_lowercase().contains(&query_lower) {
                matching_papers.push(paper);
                if matching_papers.len() >= limit {
                    break;
                }
                continue;
            }

            // Check abstract
            if paper.abstract_text.to_lowercase().contains(&query_lower) {
                matching_papers.push(paper);
                if matching_papers.len() >= limit {
                    break;
                }
                continue;
            }

            // Check subject areas
            if paper
                .subject_areas
                .iter()
                .any(|area| area.to_lowercase().contains(&query_lower))
            {
                matching_papers.push(paper);
                if matching_papers.len() >= limit {
                    break;
                }
                continue;
            }

            // Check author names
            if paper
                .authors
                .iter()
                .any(|author| author.name.to_lowercase().contains(&query_lower))
            {
                matching_papers.push(paper);
                if matching_papers.len() >= limit {
                    break;
                }
                continue;
            }
        }

        Ok(matching_papers)
    }

    /// Get the list of all paper IDs from the index
    pub fn get_paper_ids(&self) -> Result<Vec<String>> {
        self.load_paper_index()?;

        let index = self.paper_index.read();
        let index = index.as_ref().expect("Index should be loaded");

        Ok(index.papers.iter().map(|entry| entry.id.clone()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a test config pointing to the actual data directory
    fn create_test_config() -> AppConfig {
        // Tests use data copied to the destination repository
        AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8000,
            cors_origins: vec![],
            data_dir: "backend/src/data".to_string(),
            cache_dir: "target/test_cache".to_string(),
        }
    }

    #[test]
    fn test_data_loader_new() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        assert!(loader.papers_dir.ends_with("papers_by_id"));
        assert!(loader.embeddings_dir.ends_with("embeddings_by_id"));
    }

    #[test]
    fn test_load_paper_index() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let result = loader.load_paper_index();
        assert!(result.is_ok(), "Failed to load index: {:?}", result.err());

        // Verify index is loaded
        let index = loader.paper_index.read();
        assert!(index.is_some());

        let index = index.as_ref().unwrap();
        assert!(!index.papers.is_empty(), "Index should contain papers");
        assert_eq!(index.papers.len(), 1008, "Expected 1008 papers in index");
    }

    #[test]
    fn test_get_paper_by_id() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        // Load a known paper
        let paper = loader.get_paper_by_id("miccai-1274").unwrap();
        assert!(paper.is_some());

        let paper = paper.unwrap();
        assert_eq!(paper.id, "miccai-1274");
        assert!(paper.title.contains("VMRA-MaR"));
        assert!(paper.abstract_text.contains("Breast cancer"));
        assert!(!paper.authors.is_empty());

        // Test caching - second access should use cache
        let paper2 = loader.get_paper_by_id("miccai-1274").unwrap();
        assert!(paper2.is_some());
        assert_eq!(paper2.unwrap().id, "miccai-1274");
    }

    #[test]
    fn test_get_paper_by_id_not_found() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let paper = loader.get_paper_by_id("nonexistent-id").unwrap();
        assert!(paper.is_none());
    }

    #[test]
    fn test_get_all_papers() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let papers = loader.get_all_papers().unwrap();
        assert!(!papers.is_empty());
        assert_eq!(papers.len(), 1008, "Expected 1008 papers");

        // Verify first paper has required fields
        let first_paper = &papers[0];
        assert!(!first_paper.id.is_empty());
        assert!(!first_paper.title.is_empty());
    }

    #[test]
    fn test_get_embedding_by_id() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        // Load a known embedding
        let embedding = loader.get_embedding_by_id("miccai-0002").unwrap();
        assert!(embedding.is_some());

        let embedding = embedding.unwrap();
        assert!(!embedding.is_empty(), "Embedding should have elements");

        // Embeddings should be 1D arrays
        assert_eq!(embedding.ndim(), 1);

        // Test caching
        let embedding2 = loader.get_embedding_by_id("miccai-0002").unwrap();
        assert!(embedding2.is_some());
    }

    #[test]
    fn test_get_embedding_not_found() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let embedding = loader.get_embedding_by_id("nonexistent-id").unwrap();
        assert!(embedding.is_none());
    }

    #[test]
    fn test_get_all_embeddings() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        // Load all embeddings
        let embeddings = loader.get_all_embeddings().unwrap();

        // Should have embeddings (not all 1008 papers may have embeddings)
        assert!(
            !embeddings.is_empty(),
            "Should have at least some embeddings"
        );

        // Verify that embeddings are valid
        for (paper_id, embedding) in &embeddings {
            assert!(!paper_id.is_empty(), "Paper ID should not be empty");
            assert!(!embedding.is_empty(), "Embedding should have elements");
            assert_eq!(embedding.ndim(), 1, "Embeddings should be 1D arrays");
        }

        // Verify specific embedding exists and matches individual load
        if embeddings.contains_key("miccai-0002") {
            let embedding_from_all = &embeddings["miccai-0002"];
            let embedding_individual = loader.get_embedding_by_id("miccai-0002").unwrap().unwrap();
            assert_eq!(
                embedding_from_all.len(),
                embedding_individual.len(),
                "Embedding from get_all_embeddings should match individual load"
            );
        }
    }

    #[test]
    fn test_search_papers() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        // Search by title keyword
        let results = loader.search_papers("breast", 5).unwrap();
        assert!(
            !results.is_empty(),
            "Should find papers with 'breast' keyword"
        );
        assert!(results.len() <= 5, "Should respect limit");

        // Verify results contain the search term
        assert!(results
            .iter()
            .any(|p| p.title.to_lowercase().contains("breast")
                || p.abstract_text.to_lowercase().contains("breast")
                || p.subject_areas
                    .iter()
                    .any(|a| a.to_lowercase().contains("breast"))));
    }

    #[test]
    fn test_search_papers_case_insensitive() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let results_lower = loader.search_papers("cancer", 10).unwrap();
        let results_upper = loader.search_papers("CANCER", 10).unwrap();

        assert_eq!(
            results_lower.len(),
            results_upper.len(),
            "Case-insensitive search should return same number of results"
        );
    }

    #[test]
    fn test_get_paper_ids() {
        let config = create_test_config();
        let loader = DataLoader::new(&config);

        let ids = loader.get_paper_ids().unwrap();
        assert!(!ids.is_empty());
        assert_eq!(ids.len(), 1008, "Expected 1008 paper IDs");

        // Verify IDs are not empty
        assert!(ids.iter().all(|id| !id.is_empty()));
    }
}
