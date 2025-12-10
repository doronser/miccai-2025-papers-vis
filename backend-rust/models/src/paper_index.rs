use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dataset information metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetInfo {
    pub total_papers: u32,
    pub generated_at: String,
    pub scraper_version: String,
    pub source_url: String,
}

/// Author statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorStats {
    pub total_author_mentions: u32,
    pub unique_authors: u32,
    pub avg_authors_per_paper: f32,
    pub max_authors: u32,
    pub min_authors: u32,
}

/// Content statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContentStats {
    pub avg_title_length: f32,
    pub max_title_length: u32,
    pub min_title_length: u32,
    pub avg_abstract_length: f32,
    pub max_abstract_length: u32,
    pub min_abstract_length: u32,
}

/// Processing statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessingStats {
    pub success_rate: f32,
    pub total_processed: u32,
    pub successful: u32,
    pub errors: u32,
}

/// Statistics about the paper dataset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Statistics {
    pub total_papers: u32,
    pub papers_with_pdf: u32,
    pub pdf_availability_rate: f32,
    pub author_stats: AuthorStats,
    pub content_stats: ContentStats,
    pub subject_distribution: HashMap<String, u32>,
    pub processing_stats: ProcessingStats,
}

/// Simplified paper metadata entry in the index
/// Contains minimal information; full details are in individual paper JSON files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaperIndexEntry {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
}

/// Master index file structure for papers_by_id/index.json
///
/// This struct deserializes the index.json file which contains:
/// - Dataset metadata and statistics
/// - List of all paper IDs with minimal metadata
///
/// Individual paper details are stored in separate JSON files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaperIndex {
    pub dataset_info: DatasetInfo,
    pub statistics: Statistics,
    pub papers: Vec<PaperIndexEntry>,
}

impl PaperIndex {
    /// Get list of all paper IDs from the index
    pub fn paper_ids(&self) -> Vec<String> {
        self.papers.iter().map(|p| p.id.clone()).collect()
    }

    /// Get the number of papers in the index
    pub fn paper_count(&self) -> usize {
        self.papers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_index_entry_serialization() {
        let entry = PaperIndexEntry {
            id: "miccai-0001".to_string(),
            title: Some("Test Paper".to_string()),
            authors: Some(vec!["Author 1".to_string(), "Author 2".to_string()]),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: PaperIndexEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_paper_index_entry_minimal() {
        let entry = PaperIndexEntry {
            id: "miccai-0001".to_string(),
            title: None,
            authors: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"id\""));
        assert!(!json.contains("\"title\""));
        assert!(!json.contains("\"authors\""));
    }

    #[test]
    fn test_paper_ids() {
        let index = PaperIndex {
            dataset_info: DatasetInfo {
                total_papers: 3,
                generated_at: "2025-01-01".to_string(),
                scraper_version: "1.0".to_string(),
                source_url: "https://example.com".to_string(),
            },
            statistics: Statistics {
                total_papers: 3,
                papers_with_pdf: 3,
                pdf_availability_rate: 100.0,
                author_stats: AuthorStats {
                    total_author_mentions: 10,
                    unique_authors: 8,
                    avg_authors_per_paper: 3.3,
                    max_authors: 5,
                    min_authors: 2,
                },
                content_stats: ContentStats {
                    avg_title_length: 100.0,
                    max_title_length: 150,
                    min_title_length: 50,
                    avg_abstract_length: 1000.0,
                    max_abstract_length: 1500,
                    min_abstract_length: 500,
                },
                subject_distribution: HashMap::new(),
                processing_stats: ProcessingStats {
                    success_rate: 100.0,
                    total_processed: 3,
                    successful: 3,
                    errors: 0,
                },
            },
            papers: vec![
                PaperIndexEntry {
                    id: "miccai-0001".to_string(),
                    title: Some("Paper 1".to_string()),
                    authors: None,
                },
                PaperIndexEntry {
                    id: "miccai-0002".to_string(),
                    title: Some("Paper 2".to_string()),
                    authors: None,
                },
                PaperIndexEntry {
                    id: "miccai-0003".to_string(),
                    title: Some("Paper 3".to_string()),
                    authors: None,
                },
            ],
        };

        let ids = index.paper_ids();
        assert_eq!(ids.len(), 3);
        assert_eq!(ids[0], "miccai-0001");
        assert_eq!(ids[1], "miccai-0002");
        assert_eq!(ids[2], "miccai-0003");
        assert_eq!(index.paper_count(), 3);
    }
}
