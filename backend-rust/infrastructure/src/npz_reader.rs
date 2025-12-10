// NPZ file reading utilities for loading embeddings from NumPy compressed archives
//
// NPZ files are ZIP archives containing NumPy .npy arrays. The Python data pipeline
// generates .npz files with the following structure:
// - embedding.npy: The embedding vector (1D float32 array)
// - Optional metadata fields (paper_id, model_name, generated_at, text_hash)
//
// This module uses the `zip` crate to read the archive and `ndarray-npy` to deserialize
// the .npy array data into Rust ndarray types.

use ndarray::Array1;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use thiserror::Error;
use zip::ZipArchive;

/// Errors that can occur during NPZ file reading
#[derive(Error, Debug)]
pub enum NpzError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("ZIP archive error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("NPY parsing error: {0}")]
    NpyError(String),

    #[error("Embedding not found in NPZ archive")]
    EmbeddingNotFound,

    #[error("Invalid embedding format: {0}")]
    InvalidFormat(String),
}

/// NPZ embedding data structure
///
/// Contains the embedding vector and optional metadata fields.
/// The embedding vector is the primary data, while metadata can be used
/// for validation and diagnostics.
#[derive(Debug, Clone)]
pub struct NpzEmbedding {
    /// The embedding vector (typically 768 dimensions for SciBERT)
    pub embedding: Array1<f32>,

    /// Optional metadata fields (currently not extracted from NPZ)
    /// These could be extracted from separate metadata files if present
    pub paper_id: Option<String>,
    pub model_name: Option<String>,
}

/// Read an NPZ embedding file and extract the embedding vector
///
/// NPZ files are ZIP archives containing .npy files. This function:
/// 1. Opens the .npz file as a ZIP archive
/// 2. Searches for the "embedding.npy" entry in the archive
/// 3. Deserializes the .npy data into an ndarray Array1<f32>
///
/// # Arguments
///
/// * `path` - Path to the .npz file
///
/// # Returns
///
/// * `Ok(NpzEmbedding)` - Successfully loaded embedding with vector data
/// * `Err(NpzError)` - Failed to read or parse the NPZ file
///
/// # Example
///
/// ```no_run
/// use infrastructure::npz_reader::read_npz_embedding;
/// use std::path::Path;
///
/// let embedding = read_npz_embedding(Path::new("./data/embeddings_by_id/miccai-0001_embedding.npz")).unwrap();
/// println!("Embedding dimensions: {}", embedding.embedding.len());
/// ```
pub fn read_npz_embedding(path: &Path) -> Result<NpzEmbedding, NpzError> {
    // Open the NPZ file (which is a ZIP archive)
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;

    // Search for the embedding.npy file in the archive
    // NPZ files store arrays with the key as the filename (e.g., "embedding.npy")
    let mut embedding_data: Option<Array1<f32>> = None;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Look for the embedding array
        // NumPy saves arrays as "{name}.npy" in the ZIP archive
        if name == "embedding.npy" || name == "embedding" {
            // Extract the .npy file to a temporary location
            // ndarray-npy 0.7 requires reading from a file path
            // Use a unique filename with timestamp to avoid race conditions
            let temp_dir = std::env::temp_dir();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let temp_file_path = temp_dir.join(format!("embedding_{}_{}.npy", std::process::id(), timestamp));

            {
                let mut temp_file = File::create(&temp_file_path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                temp_file.write_all(&buffer)?;
            }

            // Parse the .npy format from the temporary file
            let array = ndarray_npy::read_npy(&temp_file_path)
                .map_err(|e| NpzError::NpyError(format!("Failed to parse NPY data: {}", e)))?;

            // Clean up temporary file
            let _ = std::fs::remove_file(&temp_file_path);

            embedding_data = Some(array);
            break;
        }
    }

    // Ensure we found the embedding
    let embedding = embedding_data.ok_or(NpzError::EmbeddingNotFound)?;

    // Validate embedding dimensions
    if embedding.is_empty() {
        return Err(NpzError::InvalidFormat(
            "Embedding array is empty".to_string(),
        ));
    }

    Ok(NpzEmbedding {
        embedding,
        paper_id: None,
        model_name: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_read_npz_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/file.npz");
        let result = read_npz_embedding(&path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NpzError::IoError(_)));
    }

    #[test]
    fn test_npz_embedding_struct() {
        use ndarray::arr1;

        let embedding = NpzEmbedding {
            embedding: arr1(&[0.1, 0.2, 0.3]),
            paper_id: Some("test-001".to_string()),
            model_name: Some("scibert".to_string()),
        };

        assert_eq!(embedding.embedding.len(), 3);
        assert_eq!(embedding.paper_id, Some("test-001".to_string()));
        assert_eq!(embedding.model_name, Some("scibert".to_string()));
    }
}
