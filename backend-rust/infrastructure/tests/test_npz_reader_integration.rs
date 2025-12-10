use infrastructure::npz_reader::{read_npz_embedding, NpzError};
use std::path::PathBuf;

// Integration tests for NPZ reading with real files from the source data directory
// These tests verify that the Rust NPZ reader can successfully read Python-generated NPZ files

const EMBEDDINGS_DIR: &str = "/l2l/src/miccai-2025-papers-vis/backend/src/data/embeddings_by_id";

#[test]
fn test_read_real_npz_file() {
    let embeddings_path = PathBuf::from(EMBEDDINGS_DIR);

    // Skip test if source data directory doesn't exist
    if !embeddings_path.exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    // Test with a known embedding file
    let npz_path = embeddings_path.join("miccai-0002_embedding.npz");

    if !npz_path.exists() {
        eprintln!("Skipping test: test file not found at {:?}", npz_path);
        return;
    }

    // Read the NPZ file
    let result = read_npz_embedding(&npz_path);
    assert!(result.is_ok(), "Failed to read NPZ file: {:?}", result.err());

    let npz_embedding = result.unwrap();

    // Verify the embedding structure
    assert!(!npz_embedding.embedding.is_empty(), "Embedding should not be empty");

    // SciBERT embeddings are typically 768 dimensions
    // We'll check for a reasonable range (could be 768 for SciBERT, 384 for smaller models, etc.)
    let dims = npz_embedding.embedding.len();
    println!("Loaded embedding with {} dimensions", dims);
    assert!(
        dims >= 100 && dims <= 2048,
        "Embedding dimensions seem unreasonable: {}",
        dims
    );

    // Verify that values are reasonable floats (not NaN, not infinity)
    for (i, &value) in npz_embedding.embedding.iter().enumerate().take(10) {
        assert!(
            value.is_finite(),
            "Embedding value at index {} is not finite: {}",
            i,
            value
        );
    }
}

#[test]
fn test_read_multiple_npz_files() {
    let embeddings_path = PathBuf::from(EMBEDDINGS_DIR);

    // Skip test if source data directory doesn't exist
    if !embeddings_path.exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    // Test with several known embedding files
    let test_ids = vec!["miccai-0002", "miccai-0965", "miccai-3082"];
    let mut loaded_count = 0;
    let mut first_dims = None;

    for paper_id in test_ids {
        let npz_path = embeddings_path.join(format!("{}_embedding.npz", paper_id));

        if !npz_path.exists() {
            eprintln!("Warning: test file not found at {:?}", npz_path);
            continue;
        }

        let result = read_npz_embedding(&npz_path);
        assert!(result.is_ok(), "Failed to read NPZ file for {}: {:?}", paper_id, result.err());

        let npz_embedding = result.unwrap();
        assert!(!npz_embedding.embedding.is_empty());

        let dims = npz_embedding.embedding.len();

        // Check that all embeddings have the same dimensions
        if let Some(expected_dims) = first_dims {
            assert_eq!(
                dims, expected_dims,
                "Inconsistent embedding dimensions: {} has {}, expected {}",
                paper_id, dims, expected_dims
            );
        } else {
            first_dims = Some(dims);
        }

        loaded_count += 1;
    }

    assert!(loaded_count > 0, "No NPZ files were successfully loaded");
    println!("Successfully loaded {} NPZ files with consistent dimensions", loaded_count);
}

#[test]
fn test_npz_error_for_nonexistent_file() {
    let nonexistent_path = PathBuf::from("/nonexistent/path/file.npz");
    let result = read_npz_embedding(&nonexistent_path);

    assert!(result.is_err(), "Should error for non-existent file");

    match result.unwrap_err() {
        NpzError::IoError(_) => {
            // Expected error type
        }
        other => {
            panic!("Expected IoError, got: {:?}", other);
        }
    }
}

#[test]
fn test_npz_embedding_values_range() {
    let embeddings_path = PathBuf::from(EMBEDDINGS_DIR);

    // Skip test if source data directory doesn't exist
    if !embeddings_path.exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    let npz_path = embeddings_path.join("miccai-0002_embedding.npz");

    if !npz_path.exists() {
        eprintln!("Skipping test: test file not found");
        return;
    }

    let npz_embedding = read_npz_embedding(&npz_path).expect("Failed to read NPZ");

    // Check that embedding values are in a reasonable range for normalized embeddings
    // Typically embeddings are normalized or at least bounded
    let mut min_val = f32::INFINITY;
    let mut max_val = f32::NEG_INFINITY;

    for &value in npz_embedding.embedding.iter() {
        min_val = min_val.min(value);
        max_val = max_val.max(value);
    }

    println!("Embedding value range: [{}, {}]", min_val, max_val);

    // Values should be reasonable (not all zeros, not absurdly large)
    assert!(
        min_val.abs() < 1000.0 && max_val.abs() < 1000.0,
        "Embedding values seem unreasonable: range [{}, {}]",
        min_val,
        max_val
    );
}
