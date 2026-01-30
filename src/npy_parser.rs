/// Simple NPY format parser for float32 1D arrays
///
/// NPY format structure:
/// - Magic number: b'\x93NUMPY' (6 bytes)
/// - Version: major, minor (2 bytes)
/// - Header length: 2 or 4 bytes depending on version
/// - Header: Python literal describing the array (ASCII)
/// - Data: raw binary data
use anyhow::{anyhow, Context, Result};
use ndarray::Array1;
use std::collections::HashMap;

/// Parse a 1D float32 array from NPY format bytes
pub fn parse_npy_f32(data: &[u8]) -> Result<Array1<f32>> {
    // Check magic number
    if data.len() < 10 {
        return Err(anyhow!("NPY data too short"));
    }

    if &data[0..6] != b"\x93NUMPY" {
        return Err(anyhow!("Invalid NPY magic number"));
    }

    let major = data[6];
    let minor = data[7];

    if major > 3 {
        return Err(anyhow!("Unsupported NPY version: {}.{}", major, minor));
    }

    // Parse header length
    let (header_len, data_start) = if major == 1 {
        // Version 1.0: header length is 2 bytes, little-endian
        let header_len = u16::from_le_bytes([data[8], data[9]]) as usize;
        (header_len, 10)
    } else {
        // Version 2.0+: header length is 4 bytes, little-endian
        let header_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        (header_len, 12)
    };

    // Read and parse header
    let header_end = data_start + header_len;
    if data.len() < header_end {
        return Err(anyhow!("NPY data truncated at header"));
    }

    let header_bytes = &data[data_start..header_end];
    let header_str = std::str::from_utf8(header_bytes).context("Header is not valid UTF-8")?;

    // Parse header to extract dtype and shape
    let header_dict = parse_header(header_str)?;

    // Verify dtype is float32
    let dtype = header_dict
        .get("descr")
        .ok_or_else(|| anyhow!("No 'descr' in NPY header"))?;

    if dtype != "<f4" && dtype != "=f4" && dtype != "|f4" {
        return Err(anyhow!(
            "Unsupported dtype: {}. Expected <f4 (float32)",
            dtype
        ));
    }

    // Parse shape - we expect 1D arrays
    let shape_str = header_dict
        .get("shape")
        .ok_or_else(|| anyhow!("No 'shape' in NPY header"))?;

    let shape = parse_shape(shape_str)?;
    if shape.len() != 1 {
        return Err(anyhow!("Expected 1D array, got {}D", shape.len()));
    }

    let n_elements = shape[0];

    // Read the data
    let data_bytes = &data[header_end..];
    let expected_bytes = n_elements * 4; // 4 bytes per f32

    if data_bytes.len() < expected_bytes {
        return Err(anyhow!(
            "NPY data truncated: expected {} bytes, got {}",
            expected_bytes,
            data_bytes.len()
        ));
    }

    // Convert bytes to f32 array
    let mut values = Vec::with_capacity(n_elements);
    for i in 0..n_elements {
        let offset = i * 4;
        let bytes = [
            data_bytes[offset],
            data_bytes[offset + 1],
            data_bytes[offset + 2],
            data_bytes[offset + 3],
        ];
        values.push(f32::from_le_bytes(bytes));
    }

    Ok(Array1::from_vec(values))
}

/// Parse the Python dictionary-like header
fn parse_header(header: &str) -> Result<HashMap<String, String>> {
    let mut result = HashMap::new();

    // The header is a Python literal like: {'descr': '<f4', 'fortran_order': False, 'shape': (768,), }
    // We'll do simple parsing
    let header = header.trim();

    // Find key-value pairs
    // This is a simplified parser that works for NPY headers
    if let Some(start) = header.find('{') {
        if let Some(end) = header.rfind('}') {
            let content = &header[start + 1..end];

            // Split by commas, but be careful with nested structures
            let parts: Vec<&str> = split_by_comma(content);

            for part in parts {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }

                // Find the colon separator
                if let Some(colon_pos) = part.find(':') {
                    let key = part[..colon_pos]
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"');
                    let value = part[colon_pos + 1..].trim();

                    // Extract the value, removing quotes
                    let value = value.trim_matches('\'').trim_matches('"');

                    result.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    Ok(result)
}

/// Split by comma, respecting parentheses
fn split_by_comma(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut paren_depth = 0;

    for (i, ch) in s.char_indices() {
        match ch {
            '(' | '[' | '{' => paren_depth += 1,
            ')' | ']' | '}' => paren_depth -= 1,
            ',' if paren_depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }

    if start < s.len() {
        parts.push(&s[start..]);
    }

    parts
}

/// Parse shape tuple like "(768,)" or "(100, 200)"
fn parse_shape(shape_str: &str) -> Result<Vec<usize>> {
    let shape_str = shape_str.trim();
    if !shape_str.starts_with('(') || !shape_str.ends_with(')') {
        return Err(anyhow!("Invalid shape format: {}", shape_str));
    }

    let inner = &shape_str[1..shape_str.len() - 1];
    let mut dimensions = Vec::new();

    for part in inner.split(',') {
        let part = part.trim();
        if !part.is_empty() {
            dimensions.push(
                part.parse::<usize>()
                    .with_context(|| format!("Failed to parse shape dimension: {}", part))?,
            );
        }
    }

    Ok(dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shape() {
        assert_eq!(parse_shape("(768,)").unwrap(), vec![768]);
        assert_eq!(parse_shape("(100, 200)").unwrap(), vec![100, 200]);
        assert_eq!(parse_shape("()").unwrap(), Vec::<usize>::new());
    }

    #[test]
    fn test_parse_shape_invalid_format() {
        // Missing parentheses
        assert!(parse_shape("768").is_err());
        assert!(parse_shape("[768]").is_err());
    }

    #[test]
    fn test_parse_header() {
        let header = "{'descr': '<f4', 'fortran_order': False, 'shape': (768,), }";
        let result = parse_header(header).unwrap();

        assert_eq!(result.get("descr"), Some(&"<f4".to_string()));
        assert_eq!(result.get("fortran_order"), Some(&"False".to_string()));
        assert_eq!(result.get("shape"), Some(&"(768,)".to_string()));
    }

    #[test]
    fn test_parse_header_with_different_dtypes() {
        // Test with different byte orders
        let header_le = "{'descr': '<f4', 'fortran_order': False, 'shape': (100,), }";
        let result_le = parse_header(header_le).unwrap();
        assert_eq!(result_le.get("descr"), Some(&"<f4".to_string()));

        let header_eq = "{'descr': '=f4', 'fortran_order': False, 'shape': (100,), }";
        let result_eq = parse_header(header_eq).unwrap();
        assert_eq!(result_eq.get("descr"), Some(&"=f4".to_string()));
    }

    #[test]
    fn test_split_by_comma() {
        // Simple case
        let parts = split_by_comma("a, b, c");
        assert_eq!(parts, vec!["a", " b", " c"]);

        // With nested parentheses
        let parts = split_by_comma("'shape': (768,), 'descr': '<f4'");
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_parse_npy_invalid_magic() {
        // Data with invalid magic number
        let data = b"INVALID_DATA_HERE";
        let result = parse_npy_f32(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_npy_too_short() {
        // Data that's too short
        let data = b"\x93NUMP";
        let result = parse_npy_f32(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_npy_with_real_embedding_file() {
        use std::fs::File;
        use std::io::Read;

        // Read a real embedding NPZ file and extract the NPY content
        let npz_path = "backend/src/data/embeddings_by_id/miccai-0002_embedding.npz";

        // Skip test if file doesn't exist
        let Ok(file) = File::open(npz_path) else {
            println!("Skipping test: embedding file not found");
            return;
        };

        // NPZ files are ZIP archives
        let Ok(mut archive) = zip::ZipArchive::new(file) else {
            println!("Skipping test: failed to read NPZ archive");
            return;
        };

        let Ok(mut npy_file) = archive.by_name("embedding.npy") else {
            println!("Skipping test: embedding.npy not found in archive");
            return;
        };

        let mut buffer = Vec::new();
        npy_file
            .read_to_end(&mut buffer)
            .expect("Failed to read NPY data");

        // Parse the NPY data
        let embedding = parse_npy_f32(&buffer).expect("Failed to parse NPY data");

        // Verify the embedding has expected properties
        assert!(!embedding.is_empty(), "Embedding should not be empty");
        assert_eq!(embedding.ndim(), 1, "Embedding should be 1D");

        // SciBERT embeddings typically have 768 dimensions
        assert_eq!(
            embedding.len(),
            768,
            "SciBERT embedding should have 768 dimensions"
        );

        // Verify values are reasonable (not NaN, not Inf, within reasonable range)
        for (i, &value) in embedding.iter().enumerate() {
            assert!(!value.is_nan(), "Embedding value at index {} is NaN", i);
            assert!(
                !value.is_infinite(),
                "Embedding value at index {} is infinite",
                i
            );
            assert!(
                value.abs() < 100.0,
                "Embedding value at index {} is unexpectedly large: {}",
                i,
                value
            );
        }
    }

    #[test]
    fn test_parse_npy_version_1() {
        // Create a minimal valid NPY v1.0 file in memory
        // Magic: \x93NUMPY
        // Version: 1.0
        // Header length: 2 bytes (little-endian)
        // Header: {'descr': '<f4', 'fortran_order': False, 'shape': (2,), }
        // Data: 2 float32 values

        let header = b"{'descr': '<f4', 'fortran_order': False, 'shape': (2,), }";
        // Pad header to multiple of 64 bytes (NPY requirement for alignment)
        let padding_needed = 64 - ((10 + header.len()) % 64);
        let padded_header_len = header.len() + padding_needed;

        let mut data = Vec::new();
        data.extend_from_slice(b"\x93NUMPY"); // Magic
        data.push(1); // Major version
        data.push(0); // Minor version
        data.extend_from_slice(&(padded_header_len as u16).to_le_bytes()); // Header length

        data.extend_from_slice(header);
        // Add padding (spaces and newline at end)
        for _ in 0..padding_needed - 1 {
            data.push(b' ');
        }
        data.push(b'\n');

        // Add two float32 values: 1.5 and 2.5
        data.extend_from_slice(&1.5_f32.to_le_bytes());
        data.extend_from_slice(&2.5_f32.to_le_bytes());

        let result = parse_npy_f32(&data).expect("Failed to parse synthetic NPY data");

        assert_eq!(result.len(), 2);
        assert!((result[0] - 1.5).abs() < 1e-6);
        assert!((result[1] - 2.5).abs() < 1e-6);
    }
}
