//! File I/O utilities for papers and embeddings
//!
//! This module provides utilities for:
//! - Reading JSON files with proper error context
//! - Reading NPZ embedding files
//! - Loading the paper index
//!
//! All file operations include rich error context to aid in debugging.

use crate::error::{InfraError, InfraResult};
use ndarray_npy::ReadNpyExt;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::{debug, warn};

/// Read and deserialize a JSON file
///
/// # Type Parameters
/// - `T`: The type to deserialize into, must implement `DeserializeOwned`
///
/// # Arguments
/// - `path`: Path to the JSON file to read
///
/// # Returns
/// - `Ok(T)`: Successfully deserialized data
/// - `Err(InfraError)`: File not found, I/O error, or JSON parsing error
///
/// # Example
/// ```ignore
/// use infrastructure::file_io::read_json_file;
/// use serde::{Deserialize, Serialize};
/// use std::path::Path;
///
/// #[derive(Deserialize, Serialize)]
/// struct Paper {
///     id: String,
///     title: String,
/// }
///
/// let paper: Paper = read_json_file(Path::new("paper_123.json"))?;
/// ```
pub fn read_json_file<T: DeserializeOwned>(path: &Path) -> InfraResult<T> {
    debug!("Reading JSON file: {}", path.display());

    // Check if file exists first for better error messages
    if !path.exists() {
        warn!("JSON file not found: {}", path.display());
        return Err(InfraError::not_found(path));
    }

    // Open the file
    let file = File::open(path).map_err(|e| InfraError::io(path, e))?;

    let reader = BufReader::new(file);

    // Deserialize JSON
    serde_json::from_reader(reader).map_err(|e| InfraError::json(path, e))
}

/// Read an NPZ embedding file
///
/// Reads a NumPy `.npz` archive and extracts the `embedding` array.
///
/// # Arguments
/// - `path`: Path to the NPZ file
///
/// # Returns
/// - `Ok(Array1<f32>)`: The embedding vector
/// - `Err(InfraError)`: File not found, ZIP error, or missing embedding array
///
/// # Example
/// ```ignore
/// use infrastructure::file_io::read_npz_embedding;
/// use std::path::Path;
///
/// let embedding = read_npz_embedding(Path::new("paper_123_embedding.npz"))?;
/// println!("Embedding dimensions: {}", embedding.len());
/// ```
pub fn read_npz_embedding(path: &Path) -> InfraResult<ndarray::Array1<f32>> {
    debug!("Reading NPZ embedding file: {}", path.display());

    // Check if file exists
    if !path.exists() {
        warn!("NPZ file not found: {}", path.display());
        return Err(InfraError::not_found(path));
    }

    // Open the NPZ file as a ZIP archive
    let file = File::open(path).map_err(|e| InfraError::io(path, e))?;

    let mut archive = zip::ZipArchive::new(file).map_err(|e| InfraError::zip(path, e))?;

    // Look for the 'embedding.npy' file in the archive
    let embedding_file = archive
        .by_name("embedding.npy")
        .map_err(|e| InfraError::npz(path, format!("Missing 'embedding.npy' in archive: {}", e)))?;

    // Parse the NPY file directly from the reader
    let array: ndarray::Array1<f32> = ndarray::Array1::read_npy(embedding_file)
        .map_err(|e| InfraError::npz(path, format!("Failed to parse NPY data: {}", e)))?;

    debug!("Successfully read embedding with {} dimensions", array.len());

    Ok(array)
}

/// Read a generic JSON value from a file
///
/// This is a convenience function for when you don't know the exact structure
/// of the JSON file ahead of time.
///
/// # Arguments
/// - `path`: Path to the JSON file
///
/// # Returns
/// - `Ok(serde_json::Value)`: The parsed JSON value
/// - `Err(InfraError)`: File not found, I/O error, or JSON parsing error
pub fn read_json_value(path: &Path) -> InfraResult<serde_json::Value> {
    read_json_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use tempfile::TempDir;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: String,
        value: i32,
    }

    #[test]
    fn test_read_json_file_success() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        // Write test data
        let test_data = TestData {
            id: "test_123".to_string(),
            value: 42,
        };
        let json = serde_json::to_string(&test_data).unwrap();
        fs::write(&file_path, json).unwrap();

        // Test reading
        let result: Result<TestData, _> = read_json_file(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_data);
    }

    #[test]
    fn test_read_json_file_not_found() {
        let result: Result<TestData, _> = read_json_file(Path::new("/nonexistent/file.json"));
        assert!(result.is_err());
        match result.unwrap_err() {
            InfraError::NotFound { .. } => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_read_json_file_invalid_json() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        // Write invalid JSON
        fs::write(&file_path, "{ invalid json }").unwrap();

        // Test reading
        let result: Result<TestData, _> = read_json_file(&file_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            InfraError::Json { .. } => {}
            _ => panic!("Expected Json error"),
        }
    }

    #[test]
    fn test_read_json_value() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        // Write test data
        fs::write(&file_path, r#"{"key": "value", "number": 123}"#).unwrap();

        // Test reading
        let result = read_json_value(&file_path);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["key"], "value");
        assert_eq!(value["number"], 123);
    }

    #[test]
    fn test_read_npz_embedding_not_found() {
        let result = read_npz_embedding(Path::new("/nonexistent/embedding.npz"));
        assert!(result.is_err());
        match result.unwrap_err() {
            InfraError::NotFound { .. } => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    // Note: Testing actual NPZ reading would require creating a valid NPZ file,
    // which is complex. This will be tested in integration tests with real data.
}
