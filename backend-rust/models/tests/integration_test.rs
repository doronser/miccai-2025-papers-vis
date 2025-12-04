//! Integration tests for domain models
//!
//! These tests verify that the Rust models can correctly deserialize actual
//! JSON files from the Python backend's data directory.

use models::{Author, ExternalLink, Paper};
use std::fs;
use std::path::PathBuf;

/// Get the path to the Python backend's data directory
fn get_data_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src/miccai-2025-papers-vis/backend/src/data")
}

#[test]
fn test_deserialize_real_paper_file() {
    let data_dir = get_data_dir();
    let paper_path = data_dir.join("papers_by_id/miccai-1274.json");

    // Skip test if the data file doesn't exist (e.g., in CI without data)
    if !paper_path.exists() {
        eprintln!("Skipping test: paper file not found at {:?}", paper_path);
        return;
    }

    let json = fs::read_to_string(&paper_path)
        .expect("Failed to read paper JSON file");

    let paper: Paper = serde_json::from_str(&json)
        .expect("Failed to deserialize paper from JSON");

    // Verify key fields
    assert_eq!(paper.id, "miccai-1274");
    assert!(paper.title.contains("VMRA-MaR"));
    assert!(paper.abstract_text.contains("Breast cancer"));
    assert!(!paper.authors.is_empty());
    assert_eq!(paper.authors[0].name, "Sun, Zijun");
    assert!(paper.authors[0].affiliation.is_none());
    assert!(!paper.subject_areas.is_empty());
    assert!(!paper.external_links.is_empty());
    assert_eq!(paper.external_links[0].link_type, "pdf");
    assert!(paper.publication_date.is_some());
}

#[test]
fn test_paper_with_all_optional_fields() {
    let json = r#"{
        "id": "test-complete",
        "title": "Complete Paper Example",
        "abstract": "This paper has all optional fields populated.",
        "authors": [
            {
                "name": "Dr. Jane Smith",
                "affiliation": "MIT",
                "email": "jane@mit.edu"
            }
        ],
        "subject_areas": ["AI", "ML"],
        "external_links": [
            {
                "type": "pdf",
                "url": "https://example.com/paper.pdf",
                "description": "Full paper"
            },
            {
                "type": "doi",
                "url": "https://doi.org/10.1234/example",
                "description": null
            }
        ],
        "publication_date": "2025-01-01",
        "raw_data_source": "MICCAI 2025"
    }"#;

    let paper: Paper = serde_json::from_str(json).unwrap();
    assert_eq!(paper.authors[0].email, Some("jane@mit.edu".to_string()));
    assert_eq!(paper.external_links.len(), 2);
    assert_eq!(paper.external_links[1].description, None);
}

#[test]
fn test_paper_with_minimal_fields() {
    let json = r#"{
        "id": "test-minimal",
        "title": "Minimal Paper",
        "abstract": "Only required fields.",
        "authors": [],
        "subject_areas": [],
        "external_links": []
    }"#;

    let paper: Paper = serde_json::from_str(json).unwrap();
    assert_eq!(paper.id, "test-minimal");
    assert!(paper.authors.is_empty());
    assert!(paper.publication_date.is_none());
    assert!(paper.raw_data_source.is_none());
}

#[test]
fn test_author_with_null_fields() {
    let json = r#"{
        "name": "John Doe",
        "affiliation": null,
        "email": null
    }"#;

    let author: Author = serde_json::from_str(json).unwrap();
    assert_eq!(author.name, "John Doe");
    assert!(author.affiliation.is_none());
    assert!(author.email.is_none());
}

#[test]
fn test_external_link_type_field() {
    let json = r#"{
        "type": "arxiv",
        "url": "https://arxiv.org/abs/1234.5678",
        "description": "arXiv preprint"
    }"#;

    let link: ExternalLink = serde_json::from_str(json).unwrap();
    assert_eq!(link.link_type, "arxiv");
    assert_eq!(link.url, "https://arxiv.org/abs/1234.5678");

    // Verify it serializes back with "type" field name
    let serialized = serde_json::to_string(&link).unwrap();
    assert!(serialized.contains(r#""type":"arxiv""#));
}
