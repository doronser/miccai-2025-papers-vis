use infrastructure::file_io::{read_json_file, FileIoError};
use models::{Paper, PaperIndex};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during data loading operations
#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("File I/O error: {0}")]
    FileIoError(#[from] FileIoError),

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
/// - Loading embeddings from NPZ files (future task)
/// - Caching loaded data in memory (future task)
///
/// # Example
///
/// ```no_run
/// use services::data_loader::DataLoader;
/// use std::path::PathBuf;
///
/// let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
/// let index = loader.load_paper_index().unwrap();
/// let paper = loader.get_paper_by_id("miccai-0001").unwrap();
/// ```
pub struct DataLoader {
    /// Path to the directory containing paper JSON files
    papers_dir: PathBuf,
}

impl DataLoader {
    /// Create a new DataLoader with the specified papers directory
    ///
    /// # Arguments
    ///
    /// * `papers_dir` - Path to the directory containing paper JSON files and index.json
    ///
    /// # Example
    ///
    /// ```
    /// use services::data_loader::DataLoader;
    /// use std::path::PathBuf;
    ///
    /// let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
    /// ```
    pub fn new(papers_dir: PathBuf) -> Self {
        Self { papers_dir }
    }

    /// Get the papers directory path
    pub fn papers_dir(&self) -> &Path {
        &self.papers_dir
    }

    /// Load the main paper index with all paper IDs and metadata
    ///
    /// The index.json file contains:
    /// - Dataset information (total papers, generation date, etc.)
    /// - Statistics about the dataset
    /// - List of all paper IDs with minimal metadata
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
    /// let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
    /// let index = loader.load_paper_index().unwrap();
    /// println!("Total papers: {}", index.dataset_info.total_papers);
    /// ```
    pub fn load_paper_index(&self) -> Result<PaperIndex, DataLoaderError> {
        let index_path = self.papers_dir.join("index.json");
        let index = read_json_file(&index_path)?;
        Ok(index)
    }

    /// Load a single paper by ID
    ///
    /// This method loads the full paper details from the individual paper JSON file
    /// located at `{papers_dir}/{paper_id}.json`.
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
    /// let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
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
        let paper_path = self.papers_dir.join(format!("{}.json", paper_id));

        // Check if file exists - if not, return None (not an error)
        if !paper_path.exists() {
            return Ok(None);
        }

        // Read and parse the paper JSON file
        let paper: Paper = read_json_file(&paper_path)?;
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
    /// let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loader_creation() {
        let loader = DataLoader::new(PathBuf::from("./data/papers_by_id"));
        assert_eq!(loader.papers_dir(), Path::new("./data/papers_by_id"));
    }

    #[test]
    fn test_data_loader_papers_dir() {
        let custom_path = PathBuf::from("/custom/path/papers");
        let loader = DataLoader::new(custom_path.clone());
        assert_eq!(loader.papers_dir(), custom_path.as_path());
    }
}
