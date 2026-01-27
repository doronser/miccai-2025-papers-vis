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
    fn test_parse_header() {
        let header = "{'descr': '<f4', 'fortran_order': False, 'shape': (768,), }";
        let result = parse_header(header).unwrap();

        assert_eq!(result.get("descr"), Some(&"<f4".to_string()));
        assert_eq!(result.get("fortran_order"), Some(&"False".to_string()));
        assert_eq!(result.get("shape"), Some(&"(768,)".to_string()));
    }
}
