//! Paper domain models
//!
//! Maps Python's `src/models/paper.py` to Rust with exact schema compatibility.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Author of a paper
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Author {
    /// Author's name
    pub name: String,
    
    /// Author's institutional affiliation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<String>,
    
    /// Author's email address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// External link to paper resources
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExternalLink {
    /// Type of link (e.g., "pdf", "doi", "arxiv")
    #[serde(rename = "type")]
    pub link_type: String,
    
    /// URL of the external resource
    pub url: String,
    
    /// Optional description of the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Academic paper
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Paper {
    /// Unique paper identifier
    pub id: String,
    
    /// Paper title
    pub title: String,
    
    /// Paper abstract
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    
    /// List of paper authors
    pub authors: Vec<Author>,
    
    /// Subject areas or keywords
    #[serde(default)]
    pub subject_areas: Vec<String>,
    
    /// External links (PDFs, DOIs, etc.)
    #[serde(default)]
    pub external_links: Vec<ExternalLink>,
    
    /// Publication date (ISO format, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    
    /// Source of raw data (e.g., "MICCAI 2025", optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data_source: Option<String>,
}

/// Paper with similarity score
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaperSimilarity {
    /// ID of the similar paper
    pub paper_id: String,

    /// Similarity score (0.0 to 1.0)
    pub similarity_score: f64,

    /// Full paper object
    pub paper: Paper,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author_serialization() {
        let author = Author {
            name: "John Doe".to_string(),
            affiliation: Some("University".to_string()),
            email: None,
        };

        let json = serde_json::to_string(&author).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("University"));
        assert!(!json.contains("email"));
    }

    #[test]
    fn test_paper_serialization() {
        let paper = Paper {
            id: "test-123".to_string(),
            title: "Test Paper".to_string(),
            abstract_text: "This is a test abstract.".to_string(),
            authors: vec![],
            subject_areas: vec![],
            external_links: vec![],
            publication_date: None,
            raw_data_source: None,
        };

        let json = serde_json::to_string(&paper).unwrap();
        assert!(json.contains("test-123"));
        assert!(json.contains("Test Paper"));
        assert!(json.contains("abstract"));
    }

    #[test]
    fn test_deserialize_real_paper() {
        // Sample JSON from backend/src/data/papers_by_id/miccai-1274.json
        let json = r#"{
  "id": "miccai-1274",
  "title": "VMRA-MaR: An Asymmetry-Aware Temporal Framework for Longitudinal Breast Cancer Risk Prediction",
  "abstract": "Breast cancer remains a leading cause of mortality worldwide...",
  "authors": [
    {
      "name": "Sun, Zijun",
      "affiliation": null,
      "email": null
    },
    {
      "name": "Thrun, Solveig",
      "affiliation": null,
      "email": null
    }
  ],
  "subject_areas": [
    "Body -> Breast",
    "Modalities -> CT / X-Ray"
  ],
  "external_links": [
    {
      "type": "pdf",
      "url": "https://papers.miccai.org/miccai-2025/paper/1274_paper.pdf",
      "description": "Full paper PDF"
    }
  ],
  "publication_date": "2025-10-01",
  "raw_data_source": "{}"
}"#;

        let paper: Paper = serde_json::from_str(json).unwrap();
        assert_eq!(paper.id, "miccai-1274");
        assert_eq!(paper.authors.len(), 2);
        assert_eq!(paper.authors[0].name, "Sun, Zijun");
        assert!(paper.authors[0].affiliation.is_none());
        assert_eq!(paper.subject_areas.len(), 2);
        assert_eq!(paper.external_links.len(), 1);
        assert_eq!(paper.external_links[0].link_type, "pdf");
        assert_eq!(paper.publication_date, Some("2025-10-01".to_string()));
    }

    #[test]
    fn test_paper_round_trip() {
        let original = Paper {
            id: "test-123".to_string(),
            title: "Test Paper".to_string(),
            abstract_text: "This is a test abstract.".to_string(),
            authors: vec![
                Author {
                    name: "John Doe".to_string(),
                    affiliation: Some("University".to_string()),
                    email: None,
                }
            ],
            subject_areas: vec!["AI".to_string(), "ML".to_string()],
            external_links: vec![
                ExternalLink {
                    link_type: "pdf".to_string(),
                    url: "https://example.com/paper.pdf".to_string(),
                    description: Some("Full paper".to_string()),
                }
            ],
            publication_date: Some("2025-01-01".to_string()),
            raw_data_source: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Paper = serde_json::from_str(&json).unwrap();

        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.title, deserialized.title);
        assert_eq!(original.abstract_text, deserialized.abstract_text);
        assert_eq!(original.authors.len(), deserialized.authors.len());
        assert_eq!(original.subject_areas, deserialized.subject_areas);
    }

    #[test]
    fn test_paper_similarity_structure() {
        let paper_sim = PaperSimilarity {
            paper_id: "test-123".to_string(),
            similarity_score: 0.95_f64,
            paper: Paper {
                id: "test-123".to_string(),
                title: "Test".to_string(),
                abstract_text: "Abstract".to_string(),
                authors: vec![],
                subject_areas: vec![],
                external_links: vec![],
                publication_date: None,
                raw_data_source: None,
            },
        };

        let json = serde_json::to_string(&paper_sim).unwrap();
        assert!(json.contains("\"paper_id\""));
        assert!(json.contains("\"similarity_score\""));
        assert!(json.contains("0.95"));
    }

    #[test]
    fn test_optional_fields_omitted() {
        let author = Author {
            name: "Jane Doe".to_string(),
            affiliation: None,
            email: None,
        };

        let json = serde_json::to_string(&author).unwrap();
        assert!(!json.contains("affiliation"));
        assert!(!json.contains("email"));
    }
}
