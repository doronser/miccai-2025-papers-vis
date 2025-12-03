use crate::{InfrastructureError, Result};
use ndarray::Array1;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// NPZ file data containing embedding and metadata
#[derive(Debug, Clone)]
pub struct NpzData {
    /// The embedding vector
    pub embedding: Array1<f32>,

    /// Paper ID
    pub paper_id: String,

    /// Model name used to generate the embedding
    pub model_name: String,

    /// Generation timestamp
    pub generated_at: String,

    /// Hash of the text used to generate the embedding
    pub text_hash: String,
}

/// Read and deserialize a JSON file
///
/// # Arguments
/// * `path` - Path to the JSON file
///
/// # Returns
/// Deserialized data of type T or an error
pub fn read_json_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

/// Read an NPZ file and extract embedding with metadata
///
/// NPZ files are ZIP archives containing multiple .npy files.
/// This function extracts:
/// - embedding.npy: The main embedding vector
/// - paper_id.npy, model_name.npy, etc.: Metadata fields
///
/// # Arguments
/// * `path` - Path to the NPZ file
///
/// # Returns
/// NpzData containing the embedding and metadata, or an error
pub fn read_npz_file(path: &Path) -> Result<NpzData> {
    let file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| InfrastructureError::NpzReadError(format!("Failed to open NPZ archive: {}", e)))?;

    // Extract embedding array
    let embedding = read_npy_array_from_zip(&mut archive, "embedding.npy")?;

    // Extract metadata fields as scalar values
    let paper_id = read_npy_string_from_zip(&mut archive, "paper_id.npy")?;
    let model_name = read_npy_string_from_zip(&mut archive, "model_name.npy")?;
    let generated_at = read_npy_string_from_zip(&mut archive, "generated_at.npy")?;
    let text_hash = read_npy_string_from_zip(&mut archive, "text_hash.npy")?;

    Ok(NpzData {
        embedding,
        paper_id,
        model_name,
        generated_at,
        text_hash,
    })
}

/// Read a .npy array from a ZIP archive
fn read_npy_array_from_zip(
    archive: &mut zip::ZipArchive<File>,
    filename: &str,
) -> Result<Array1<f32>> {
    let file = archive
        .by_name(filename)
        .map_err(|e| InfrastructureError::NpzReadError(format!("Missing '{}': {}", filename, e)))?;

    // Read NPY format directly from the zip file entry
    use ndarray_npy::ReadNpyExt;
    let array: Array1<f32> = Array1::<f32>::read_npy(file)
        .map_err(|e| InfrastructureError::NpzReadError(format!("Failed to parse '{}' as NPY: {}", filename, e)))?;

    Ok(array)
}

/// Read a string scalar from a .npy file in a ZIP archive
///
/// NumPy saves Python strings as Unicode arrays (dtype 'U') or ASCII arrays (dtype 'S').
/// For Unicode strings, NumPy uses UTF-32 encoding with a fixed width per character.
/// We need to parse the NPY header to get the string correctly.
fn read_npy_string_from_zip(
    archive: &mut zip::ZipArchive<File>,
    filename: &str,
) -> Result<String> {
    let mut file = archive
        .by_name(filename)
        .map_err(|e| InfrastructureError::NpzReadError(format!("Missing '{}': {}", filename, e)))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| InfrastructureError::NpzReadError(format!("Failed to read '{}': {}", filename, e)))?;

    // Parse the NPY header to determine the data type
    // NPY format: magic (6 bytes) + version (2 bytes) + header len (2 or 4 bytes) + header (JSON-like dict)
    if buffer.len() < 10 {
        return Err(InfrastructureError::NpzReadError(format!(
            "File '{}' too small to be NPY format",
            filename
        )));
    }

    // Find the header end (the header is followed by '\n')
    let header_start = 10; // After magic + version + header_len
    let header_end = buffer[header_start..]
        .iter()
        .position(|&b| b == b'\n')
        .map(|pos| header_start + pos)
        .ok_or_else(|| InfrastructureError::NpzReadError(format!(
            "Invalid NPY header in '{}'",
            filename
        )))?;

    let data_start = header_end + 1;

    // Extract the raw data bytes (after the header)
    let data_bytes = &buffer[data_start..];

    // For Unicode strings, NumPy stores them as UTF-32LE with null padding
    // We'll decode them as UTF-32LE and trim null bytes
    if let Ok(string) = decode_utf32le(data_bytes) {
        return Ok(string);
    }

    // Fallback: try ASCII/UTF-8
    let string = String::from_utf8(data_bytes.iter().copied().filter(|&b| b != 0).collect())
        .map_err(|e| InfrastructureError::NpzReadError(format!("Failed to decode '{}': {}", filename, e)))?;

    Ok(string)
}

/// Decode UTF-32LE bytes to a string
fn decode_utf32le(bytes: &[u8]) -> Result<String> {
    // UTF-32 uses 4 bytes per character
    if bytes.len() % 4 != 0 {
        return Err(InfrastructureError::NpzReadError(
            "Invalid UTF-32 data (length not multiple of 4)".to_string(),
        ));
    }

    let mut chars = Vec::new();
    for chunk in bytes.chunks_exact(4) {
        let codepoint = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        if codepoint == 0 {
            break; // Null terminator
        }
        if let Some(c) = char::from_u32(codepoint) {
            chars.push(c);
        }
    }

    Ok(chars.into_iter().collect())
}

/// Read the paper index file
///
/// Convenience function to read the index.json file in the papers directory.
///
/// # Arguments
/// * `papers_dir` - Path to the papers directory
///
/// # Returns
/// Deserialized paper index or an error
pub fn read_paper_index(papers_dir: &Path) -> Result<serde_json::Value> {
    let index_path = papers_dir.join("index.json");
    read_json_file(&index_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::Paper;

    #[test]
    fn test_read_json_file_paper() {
        // Test reading a real paper JSON file
        let paper_path = Path::new("/l2l/src/miccai-2025-papers-vis/backend/src/data/papers_by_id/miccai-0002.json");

        // Skip test if file doesn't exist (e.g., in CI environment)
        if !paper_path.exists() {
            eprintln!("Skipping test: Paper file not found");
            return;
        }

        let result: Result<Paper> = read_json_file(paper_path);
        assert!(result.is_ok());

        let paper = result.unwrap();
        assert_eq!(paper.id, "miccai-0002");
        assert!(paper.title.contains("Alzheimer"));
        assert_eq!(paper.authors.len(), 2);
        assert_eq!(paper.authors[0].name, "Zhang, Jianwei");
        assert_eq!(paper.subject_areas.len(), 6);
    }

    #[test]
    fn test_read_json_file_missing() {
        let result: Result<Paper> = read_json_file(Path::new("/nonexistent/file.json"));
        assert!(result.is_err());

        match result {
            Err(InfrastructureError::IoError(_)) => {
                // Expected error type
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_read_paper_index() {
        let papers_dir = Path::new("/l2l/src/miccai-2025-papers-vis/backend/src/data/papers_by_id");

        // Skip test if directory doesn't exist
        if !papers_dir.exists() {
            eprintln!("Skipping test: Papers directory not found");
            return;
        }

        let result = read_paper_index(papers_dir);
        assert!(result.is_ok());

        let index = result.unwrap();

        // Verify index structure
        assert!(index.is_object());
        assert!(index.get("dataset_info").is_some());
        assert!(index.get("statistics").is_some());

        // Check total_papers field
        if let Some(dataset_info) = index.get("dataset_info") {
            assert!(dataset_info.get("total_papers").is_some());
        }
    }

    #[test]
    fn test_read_npz_file() {
        let npz_path = Path::new("/l2l/src/miccai-2025-papers-vis/backend/src/data/embeddings_by_id/miccai-0002_embedding.npz");

        // Skip test if file doesn't exist
        if !npz_path.exists() {
            eprintln!("Skipping test: NPZ file not found");
            return;
        }

        let result = read_npz_file(npz_path);
        if let Err(ref e) = result {
            eprintln!("Error reading NPZ file: {:?}", e);
        }
        assert!(result.is_ok());

        let npz_data = result.unwrap();

        // Verify embedding shape (SciBERT embeddings are 768-dimensional)
        assert_eq!(npz_data.embedding.len(), 768);

        // Verify metadata
        assert_eq!(npz_data.paper_id, "miccai-0002");
        assert!(!npz_data.model_name.is_empty());
        assert!(!npz_data.generated_at.is_empty());
        assert!(!npz_data.text_hash.is_empty());
    }

    #[test]
    fn test_read_npz_file_missing() {
        let result = read_npz_file(Path::new("/nonexistent/file.npz"));
        assert!(result.is_err());

        match result {
            Err(InfrastructureError::IoError(_)) => {
                // Expected error type
            }
            _ => panic!("Expected IoError"),
        }
    }
}
