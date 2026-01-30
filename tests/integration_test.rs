use miccai_2025_papers_vis::models::paper::Paper;
use std::fs;

#[test]
fn test_deserialize_real_paper_file() {
    // Read the sample JSON file from the backend data directory
    let json_str = fs::read_to_string("backend/src/data/papers_by_id/miccai-1274.json")
        .expect("Failed to read sample JSON file");

    // Deserialize
    let paper: Paper =
        serde_json::from_str(&json_str).expect("Failed to deserialize paper from real file");

    // Verify the deserialized data
    assert_eq!(paper.id, "miccai-1274");
    assert_eq!(
        paper.title,
        "VMRA-MaR: An Asymmetry-Aware Temporal Framework for Longitudinal Breast Cancer Risk Prediction"
    );
    assert!(paper.abstract_text.contains("Breast cancer remains"));
    assert_eq!(paper.authors.len(), 3);
    assert_eq!(paper.authors[0].name, "Sun, Zijun");
    assert_eq!(paper.subject_areas.len(), 4);
    assert_eq!(paper.external_links.len(), 1);
    assert_eq!(paper.external_links[0].link_type, "pdf");
    assert_eq!(
        paper.external_links[0].url,
        "https://papers.miccai.org/miccai-2025/paper/1274_paper.pdf"
    );
    assert_eq!(paper.publication_date, Some("2025-10-01".to_string()));
}

#[test]
fn test_round_trip_serialization() {
    // Read the sample JSON file from the backend data directory
    let json_str = fs::read_to_string("backend/src/data/papers_by_id/miccai-1274.json")
        .expect("Failed to read sample JSON file");

    // Deserialize
    let paper: Paper =
        serde_json::from_str(&json_str).expect("Failed to deserialize paper from real file");

    // Serialize back to JSON
    let serialized = serde_json::to_string(&paper).expect("Failed to serialize paper");

    // Parse as JSON value to verify field names
    let json_value: serde_json::Value =
        serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

    // Verify critical field names match Python API
    assert!(
        json_value.get("abstract").is_some(),
        "abstract field missing in JSON output"
    );
    assert!(
        json_value.get("abstract_text").is_none(),
        "abstract_text should not be in JSON output"
    );
    assert_eq!(json_value["id"], "miccai-1274");
    assert_eq!(json_value["authors"][0]["name"], "Sun, Zijun");
    assert_eq!(json_value["external_links"][0]["type"], "pdf");
    assert!(
        json_value.get("type").is_none(),
        "top-level type field should not exist"
    );
}
