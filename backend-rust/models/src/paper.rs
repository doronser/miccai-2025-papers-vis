use serde::{Deserialize, Serialize};

/// Represents an author of a paper with optional affiliation and email.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Represents an external link (e.g., to PDF, DOI, project page).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Represents a MICCAI 2025 paper with all its metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Paper {
    pub id: String,
    pub title: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub authors: Vec<Author>,
    #[serde(default)]
    pub subject_areas: Vec<String>,
    #[serde(default)]
    pub external_links: Vec<ExternalLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_data_source: Option<String>,
}

/// Represents a paper with its similarity score to another paper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaperSimilarity {
    pub paper_id: String,
    pub similarity_score: f64,
    pub paper: Paper,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_author_serialization() {
        let author = Author {
            name: "John Doe".to_string(),
            affiliation: Some("University of Example".to_string()),
            email: Some("john@example.com".to_string()),
        };

        let json = serde_json::to_string(&author).unwrap();
        assert!(json.contains("\"name\":\"John Doe\""));
        assert!(json.contains("\"affiliation\":\"University of Example\""));
        assert!(json.contains("\"email\":\"john@example.com\""));
    }

    #[test]
    fn test_author_with_none_fields() {
        let author = Author {
            name: "Jane Doe".to_string(),
            affiliation: None,
            email: None,
        };

        let json = serde_json::to_string(&author).unwrap();
        assert!(json.contains("\"name\":\"Jane Doe\""));
        assert!(!json.contains("affiliation"));
        assert!(!json.contains("email"));
    }

    #[test]
    fn test_author_deserialization() {
        let json = r#"{"name":"John Doe","affiliation":"University of Example","email":"john@example.com"}"#;
        let author: Author = serde_json::from_str(json).unwrap();
        assert_eq!(author.name, "John Doe");
        assert_eq!(author.affiliation, Some("University of Example".to_string()));
        assert_eq!(author.email, Some("john@example.com".to_string()));
    }

    #[test]
    fn test_external_link_serialization() {
        let link = ExternalLink {
            link_type: "pdf".to_string(),
            url: "https://example.com/paper.pdf".to_string(),
            description: Some("Full paper PDF".to_string()),
        };

        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("\"type\":\"pdf\""));
        assert!(json.contains("\"url\":\"https://example.com/paper.pdf\""));
        assert!(json.contains("\"description\":\"Full paper PDF\""));
    }

    #[test]
    fn test_external_link_deserialization() {
        let json = r#"{"type":"doi","url":"https://doi.org/10.1000/example","description":null}"#;
        let link: ExternalLink = serde_json::from_str(json).unwrap();
        assert_eq!(link.link_type, "doi");
        assert_eq!(link.url, "https://doi.org/10.1000/example");
        assert_eq!(link.description, None);
    }

    #[test]
    fn test_paper_serialization() {
        let paper = Paper {
            id: "paper_001".to_string(),
            title: "Example Paper".to_string(),
            abstract_text: "This is an abstract.".to_string(),
            authors: vec![Author {
                name: "John Doe".to_string(),
                affiliation: Some("University".to_string()),
                email: None,
            }],
            subject_areas: vec!["Machine Learning".to_string()],
            external_links: vec![],
            publication_date: Some("2025-01-01".to_string()),
            raw_data_source: None,
        };

        let json = serde_json::to_string(&paper).unwrap();
        assert!(json.contains("\"id\":\"paper_001\""));
        assert!(json.contains("\"title\":\"Example Paper\""));
        assert!(json.contains("\"abstract\":\"This is an abstract.\""));
    }

    #[test]
    fn test_paper_deserialization() {
        let json = r#"{
            "id": "paper_001",
            "title": "Example Paper",
            "abstract": "This is an abstract.",
            "authors": [{"name": "John Doe", "affiliation": "University"}],
            "subject_areas": ["Machine Learning"],
            "external_links": [],
            "publication_date": "2025-01-01"
        }"#;

        let paper: Paper = serde_json::from_str(json).unwrap();
        assert_eq!(paper.id, "paper_001");
        assert_eq!(paper.title, "Example Paper");
        assert_eq!(paper.abstract_text, "This is an abstract.");
        assert_eq!(paper.authors.len(), 1);
        assert_eq!(paper.subject_areas.len(), 1);
    }

    #[test]
    fn test_paper_with_defaults() {
        let json = r#"{
            "id": "paper_002",
            "title": "Minimal Paper",
            "abstract": "Abstract text.",
            "authors": []
        }"#;

        let paper: Paper = serde_json::from_str(json).unwrap();
        assert_eq!(paper.id, "paper_002");
        assert_eq!(paper.subject_areas.len(), 0);
        assert_eq!(paper.external_links.len(), 0);
        assert_eq!(paper.publication_date, None);
    }

    #[test]
    fn test_paper_similarity_serialization() {
        let paper = Paper {
            id: "paper_001".to_string(),
            title: "Example Paper".to_string(),
            abstract_text: "Abstract.".to_string(),
            authors: vec![],
            subject_areas: vec![],
            external_links: vec![],
            publication_date: None,
            raw_data_source: None,
        };

        let similarity = PaperSimilarity {
            paper_id: "paper_001".to_string(),
            similarity_score: 0.95,
            paper,
        };

        let json = serde_json::to_string(&similarity).unwrap();
        assert!(json.contains("\"paper_id\":\"paper_001\""));
        assert!(json.contains("\"similarity_score\":0.95"));
    }

    #[test]
    fn test_paper_similarity_deserialization() {
        let json = r#"{
            "paper_id": "paper_001",
            "similarity_score": 0.85,
            "paper": {
                "id": "paper_001",
                "title": "Test Paper",
                "abstract": "Test abstract.",
                "authors": []
            }
        }"#;

        let similarity: PaperSimilarity = serde_json::from_str(json).unwrap();
        assert_eq!(similarity.paper_id, "paper_001");
        assert_eq!(similarity.similarity_score, 0.85);
        assert_eq!(similarity.paper.id, "paper_001");
    }
}
