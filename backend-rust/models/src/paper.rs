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
    pub similarity_score: f32,
    
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
}
