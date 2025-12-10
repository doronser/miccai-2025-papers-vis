use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during file I/O operations
#[derive(Error, Debug)]
pub enum FileIoError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}

/// Generic JSON file reader that deserializes into any type implementing DeserializeOwned
///
/// # Arguments
///
/// * `path` - Path to the JSON file to read
///
/// # Returns
///
/// * `Ok(T)` - Successfully deserialized data
/// * `Err(FileIoError::FileNotFound)` - File does not exist
/// * `Err(FileIoError::ReadError)` - I/O error reading the file
/// * `Err(FileIoError::ParseError)` - JSON parsing/deserialization error
///
/// # Example
///
/// ```no_run
/// use infrastructure::file_io::read_json_file;
/// use serde::Deserialize;
/// use std::path::Path;
///
/// #[derive(Deserialize)]
/// struct MyData {
///     name: String,
///     value: i32,
/// }
///
/// let data: MyData = read_json_file(Path::new("data.json")).unwrap();
/// ```
pub fn read_json_file<T: DeserializeOwned>(path: &Path) -> Result<T, FileIoError> {
    // Check if file exists first to provide a more specific error
    if !path.exists() {
        return Err(FileIoError::FileNotFound {
            path: path.display().to_string(),
        });
    }

    // Open file and create buffered reader for better performance
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Deserialize JSON
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::io::Write;
    use tempfile::TempDir;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_read_json_file_success() {
        // Create temporary directory and file
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let test_data = r#"{"name": "test", "value": 42}"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();

        // Test reading
        let result: TestData = read_json_file(&file_path).unwrap();
        assert_eq!(result.name, "test");
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_read_json_file_not_found() {
        let path = Path::new("/nonexistent/path/file.json");
        let result: Result<TestData, FileIoError> = read_json_file(path);

        assert!(result.is_err());
        match result.unwrap_err() {
            FileIoError::FileNotFound { .. } => (),
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_read_json_file_parse_error() {
        // Create temporary directory and file with invalid JSON
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.json");

        let invalid_json = r#"{"name": "test", "value": "not_a_number"}"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(invalid_json.as_bytes()).unwrap();

        // Test reading - should fail parsing
        let result: Result<TestData, FileIoError> = read_json_file(&file_path);

        assert!(result.is_err());
        match result.unwrap_err() {
            FileIoError::ParseError(_) => (),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_read_json_file_with_nested_structure() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Nested {
            inner: TestData,
        }

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nested.json");

        let nested_json = r#"{"inner": {"name": "nested", "value": 100}}"#;
        let mut file = File::create(&file_path).unwrap();
        file.write_all(nested_json.as_bytes()).unwrap();

        let result: Nested = read_json_file(&file_path).unwrap();
        assert_eq!(result.inner.name, "nested");
        assert_eq!(result.inner.value, 100);
    }
}
