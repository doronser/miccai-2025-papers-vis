use dashmap::DashMap;
use infrastructure::file_io::{read_json_file, FileIoError};
use infrastructure::npz_reader::{read_npz_embedding, NpzError};
use models::{Paper, PaperIndex};
use ndarray::Array1;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Errors that can occur during data loading operations
#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("File I/O error: {0}")]
    FileIoError(#[from] FileIoError),

    #[error("NPZ reading error: {0}")]
    NpzError(#[from] NpzError),

    #[error("Data directory not found: {0}")]
    DataDirNotFound(String),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),
}

/// DataLoader service for loading papers and embeddings from the file system
///
/// The DataLoader is responsible for:
/// - Loading the master paper index from index.json
/// - Loading individual paper JSON files
/// - Loading embeddings from NPZ files
/// - Caching loaded data in memory for fast subsequent access
///
/// # Concurrency Strategy
///
/// This implementation uses `DashMap` for lock-free concurrent access to papers
/// and embeddings caches. DashMap provides excellent performance for read-heavy
/// workloads (which is typical for this application) while maintaining thread safety.
/// The paper index cache uses `Arc<RwLock<>>` since it's loaded once at startup
/// and rarely changes.
///
/// # Example
///
/// ```no_run
/// use services::data_loader::DataLoader;
/// use std::path::PathBuf;
///
/// let loader = DataLoader::new(
///     PathBuf::from("./data/papers_by_id"),
///     PathBuf::from("./data/embeddings_by_id")
/// );
/// let index = loader.load_paper_index().unwrap();
/// let paper = loader.get_paper_by_id("miccai-0001").unwrap();
/// let embedding = loader.get_embedding_by_id("miccai-0001").unwrap();
/// ```
pub struct DataLoader {
    /// Path to the directory containing paper JSON files
    papers_dir: PathBuf,
    /// Path to the directory containing embedding NPZ files
    embeddings_dir: PathBuf,
    /// In-memory cache for papers using DashMap for lock-free concurrent access
    papers_cache: DashMap<String, Paper>,
    /// In-memory cache for embeddings using DashMap for lock-free concurrent access
    embeddings_cache: DashMap<String, Array1<f32>>,
    /// Cached paper index (loaded once and reused)
    paper_index_cache: Arc<RwLock<Option<PaperIndex>>>,
    /// Cache statistics: paper cache hits
    paper_cache_hits: AtomicU64,
    /// Cache statistics: paper cache misses
    paper_cache_misses: AtomicU64,
    /// Cache statistics: embedding cache hits
    embedding_cache_hits: AtomicU64,
    /// Cache statistics: embedding cache misses
    embedding_cache_misses: AtomicU64,
}

impl DataLoader {
    /// Create a new DataLoader with the specified papers and embeddings directories
    ///
    /// Initializes empty in-memory caches for papers, embeddings, and the paper index.
    /// Caches are populated lazily on first access.
    ///
    /// # Arguments
    ///
    /// * `papers_dir` - Path to the directory containing paper JSON files and index.json
    /// * `embeddings_dir` - Path to the directory containing embedding NPZ files
    ///
    /// # Example
    ///
    /// ```
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    /// ```
    pub fn new(papers_dir: PathBuf, embeddings_dir: PathBuf) -> Self {
        Self {
            papers_dir,
            embeddings_dir,
            papers_cache: DashMap::new(),
            embeddings_cache: DashMap::new(),
            paper_index_cache: Arc::new(RwLock::new(None)),
            paper_cache_hits: AtomicU64::new(0),
            paper_cache_misses: AtomicU64::new(0),
            embedding_cache_hits: AtomicU64::new(0),
            embedding_cache_misses: AtomicU64::new(0),
        }
    }

    /// Get the papers directory path
    pub fn papers_dir(&self) -> &Path {
        &self.papers_dir
    }

    /// Get the embeddings directory path
    pub fn embeddings_dir(&self) -> &Path {
        &self.embeddings_dir
    }

    /// Get cache statistics for papers
    ///
    /// Returns a tuple of (cache_hits, cache_misses)
    pub fn paper_cache_stats(&self) -> (u64, u64) {
        (
            self.paper_cache_hits.load(Ordering::Relaxed),
            self.paper_cache_misses.load(Ordering::Relaxed),
        )
    }

    /// Get cache statistics for embeddings
    ///
    /// Returns a tuple of (cache_hits, cache_misses)
    pub fn embedding_cache_stats(&self) -> (u64, u64) {
        (
            self.embedding_cache_hits.load(Ordering::Relaxed),
            self.embedding_cache_misses.load(Ordering::Relaxed),
        )
    }

    /// Load the main paper index with all paper IDs and metadata
    ///
    /// The index.json file contains:
    /// - Dataset information (total papers, generation date, etc.)
    /// - Statistics about the dataset
    /// - List of all paper IDs with minimal metadata
    ///
    /// This method caches the index after the first load for fast subsequent access.
    ///
    /// # Returns
    ///
    /// * `Ok(PaperIndex)` - Successfully loaded index
    /// * `Err(DataLoaderError)` - Failed to load or parse index.json
    ///
    /// # Example
    ///
    /// ```no_run
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    /// let index = loader.load_paper_index().unwrap();
    /// println!("Total papers: {}", index.dataset_info.total_papers);
    /// ```
    pub fn load_paper_index(&self) -> Result<PaperIndex, DataLoaderError> {
        // Try to get from cache first (read lock)
        {
            let cache = self.paper_index_cache.read().unwrap();
            if let Some(ref index) = *cache {
                return Ok(index.clone());
            }
        }

        // Cache miss - load from file
        let index_path = self.papers_dir.join("index.json");
        let index: PaperIndex = read_json_file(&index_path)?;

        // Store in cache (write lock)
        {
            let mut cache = self.paper_index_cache.write().unwrap();
            *cache = Some(index.clone());
        }

        Ok(index)
    }

    /// Load a single paper by ID
    ///
    /// This method loads the full paper details from the individual paper JSON file
    /// located at `{papers_dir}/{paper_id}.json`.
    ///
    /// Uses in-memory caching to avoid repeated file I/O. On cache hit, returns
    /// the cached paper immediately (sub-millisecond). On cache miss, loads from
    /// file and populates the cache.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The paper ID (e.g., "miccai-0001")
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Paper))` - Paper found and successfully loaded
    /// * `Ok(None)` - Paper ID not found (file doesn't exist)
    /// * `Err(DataLoaderError)` - I/O or parsing error occurred
    ///
    /// # Example
    ///
    /// ```no_run
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    ///
    /// // Load an existing paper
    /// let paper = loader.get_paper_by_id("miccai-0001").unwrap();
    /// if let Some(p) = paper {
    ///     println!("Title: {}", p.title);
    /// }
    ///
    /// // Try to load a non-existent paper
    /// let result = loader.get_paper_by_id("nonexistent").unwrap();
    /// assert!(result.is_none());
    /// ```
    pub fn get_paper_by_id(&self, paper_id: &str) -> Result<Option<Paper>, DataLoaderError> {
        // Check cache first
        if let Some(paper) = self.papers_cache.get(paper_id) {
            self.paper_cache_hits.fetch_add(1, Ordering::Relaxed);
            return Ok(Some(paper.clone()));
        }

        // Cache miss - load from file
        self.paper_cache_misses.fetch_add(1, Ordering::Relaxed);

        let paper_path = self.papers_dir.join(format!("{}.json", paper_id));

        // Check if file exists - if not, return None (not an error)
        if !paper_path.exists() {
            return Ok(None);
        }

        // Read and parse the paper JSON file
        let paper: Paper = read_json_file(&paper_path)?;

        // Store in cache
        self.papers_cache.insert(paper_id.to_string(), paper.clone());

        Ok(Some(paper))
    }

    /// Load all papers from the index
    ///
    /// This method:
    /// 1. Loads the master index.json
    /// 2. Iterates through all paper IDs in the index
    /// 3. Loads each individual paper JSON file
    /// 4. Returns a vector of all successfully loaded papers
    ///
    /// Papers that fail to load are skipped (not considered an error).
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Paper>)` - Vector of all successfully loaded papers
    /// * `Err(DataLoaderError)` - Failed to load index.json
    ///
    /// # Example
    ///
    /// ```no_run
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    /// let papers = loader.get_all_papers().unwrap();
    /// println!("Loaded {} papers", papers.len());
    /// ```
    pub fn get_all_papers(&self) -> Result<Vec<Paper>, DataLoaderError> {
        let index = self.load_paper_index()?;
        let mut papers = Vec::new();

        // Iterate through all paper entries in the index
        for paper_entry in &index.papers {
            // Try to load each paper by ID
            match self.get_paper_by_id(&paper_entry.id) {
                Ok(Some(paper)) => {
                    papers.push(paper);
                }
                Ok(None) => {
                    // Paper file not found - skip it
                    log::warn!("Paper file not found for ID: {}", paper_entry.id);
                }
                Err(e) => {
                    // Error loading paper - log and skip
                    log::warn!("Failed to load paper {}: {}", paper_entry.id, e);
                }
            }
        }

        Ok(papers)
    }

    /// Load a single embedding by paper ID
    ///
    /// This method loads the embedding vector from an NPZ file located at
    /// `{embeddings_dir}/{paper_id}_embedding.npz`.
    ///
    /// Uses in-memory caching to avoid repeated file I/O and NPZ parsing. On cache
    /// hit, returns the cached embedding immediately (sub-millisecond). On cache miss,
    /// loads from file and populates the cache.
    ///
    /// # Arguments
    ///
    /// * `paper_id` - The paper ID (e.g., "miccai-0001")
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Array1<f32>))` - Embedding found and successfully loaded
    /// * `Ok(None)` - Embedding file not found (not an error)
    /// * `Err(DataLoaderError)` - I/O or parsing error occurred
    ///
    /// # Example
    ///
    /// ```no_run
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    ///
    /// // Load an existing embedding
    /// let embedding = loader.get_embedding_by_id("miccai-0001").unwrap();
    /// if let Some(emb) = embedding {
    ///     println!("Embedding dimensions: {}", emb.len());
    /// }
    ///
    /// // Try to load a non-existent embedding
    /// let result = loader.get_embedding_by_id("nonexistent").unwrap();
    /// assert!(result.is_none());
    /// ```
    pub fn get_embedding_by_id(&self, paper_id: &str) -> Result<Option<Array1<f32>>, DataLoaderError> {
        // Check cache first
        if let Some(embedding) = self.embeddings_cache.get(paper_id) {
            self.embedding_cache_hits.fetch_add(1, Ordering::Relaxed);
            return Ok(Some(embedding.clone()));
        }

        // Cache miss - load from file
        self.embedding_cache_misses.fetch_add(1, Ordering::Relaxed);

        let embedding_path = self.embeddings_dir.join(format!("{}_embedding.npz", paper_id));

        // Check if file exists - if not, return None (not an error)
        if !embedding_path.exists() {
            return Ok(None);
        }

        // Read and parse the NPZ file
        let npz_embedding = read_npz_embedding(&embedding_path)?;
        let embedding = npz_embedding.embedding;

        // Store in cache
        self.embeddings_cache.insert(paper_id.to_string(), embedding.clone());

        Ok(Some(embedding))
    }

    /// Load all embeddings for papers in the index
    ///
    /// This method:
    /// 1. Loads the master index.json
    /// 2. Iterates through all paper IDs in the index
    /// 3. Loads each embedding NPZ file
    /// 4. Returns a HashMap mapping paper IDs to embedding vectors
    ///
    /// Papers without embeddings are skipped (not considered an error).
    ///
    /// # Returns
    ///
    /// * `Ok(HashMap<String, Array1<f32>>)` - Map of paper IDs to embeddings
    /// * `Err(DataLoaderError)` - Failed to load index.json
    ///
    /// # Example
    ///
    /// ```no_run
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(
    ///     PathBuf::from("./data/papers_by_id"),
    ///     PathBuf::from("./data/embeddings_by_id")
    /// );
    /// let embeddings = loader.get_all_embeddings().unwrap();
    /// println!("Loaded {} embeddings", embeddings.len());
    /// ```
    pub fn get_all_embeddings(&self) -> Result<HashMap<String, Array1<f32>>, DataLoaderError> {
        let index = self.load_paper_index()?;
        let mut embeddings = HashMap::new();

        // Iterate through all paper entries in the index
        for paper_entry in &index.papers {
            // Try to load each embedding by paper ID
            match self.get_embedding_by_id(&paper_entry.id) {
                Ok(Some(embedding)) => {
                    embeddings.insert(paper_entry.id.clone(), embedding);
                }
                Ok(None) => {
                    // Embedding file not found - skip it (not all papers have embeddings)
                    log::debug!("Embedding file not found for ID: {}", paper_entry.id);
                }
                Err(e) => {
                    // Error loading embedding - log and skip
                    log::warn!("Failed to load embedding for {}: {}", paper_entry.id, e);
                }
            }
        }

        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loader_creation() {
        let loader = DataLoader::new(
            PathBuf::from("./data/papers_by_id"),
            PathBuf::from("./data/embeddings_by_id"),
        );
        assert_eq!(loader.papers_dir(), Path::new("./data/papers_by_id"));
        assert_eq!(
            loader.embeddings_dir(),
            Path::new("./data/embeddings_by_id")
        );
    }

    #[test]
    fn test_data_loader_papers_dir() {
        let papers_path = PathBuf::from("/custom/path/papers");
        let embeddings_path = PathBuf::from("/custom/path/embeddings");
        let loader = DataLoader::new(papers_path.clone(), embeddings_path.clone());
        assert_eq!(loader.papers_dir(), papers_path.as_path());
        assert_eq!(loader.embeddings_dir(), embeddings_path.as_path());
    }
}
