use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Author information for a paper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// External link to resources related to a paper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ExternalLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Main paper model representing a research paper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data_source: Option<String>,
}

/// Paper similarity result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PaperSimilarity {
    pub paper_id: String,
    pub similarity_score: f64,
    pub paper: Paper,
}

/// Graph node for network visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub subject_areas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
}

/// Graph edge representing similarity between papers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub similarity: f64,
}

/// Complete graph data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sample paper JSON from miccai-0002.json
    const SAMPLE_PAPER_JSON: &str = r#"{
  "id": "miccai-0002",
  "title": "Surface-based Multi-Axis Longitudinal Disentanglement Using Contrastive Learning for Alzheimer's Disease",
  "abstract": "Accurate modeling of disease progression is essential for comprehending the heterogeneous neuropathologies such as Alzheimer's Disease (AD). Traditional neuroimaging analysis often confound disease effects with normal aging, complicating the differential diagnosis. Recent advancements in deep learning have catalyzed the development of disentanglement techniques in Autoencoder networks, aiming to segregate longitudinal changes attributable to aging from those due to disease-specific alterations within the latent space. However, existing longitudinal disentanglement methods usually model disease as a single axis factor which ignores the complexity and heterogeneity of Alzheimer's Disease. In response to this issue, we propose a novel Surface-based Multi-axis Disentanglement framework.This framework posits multiple disease axes within the latent space, enhancing the model's capacity to encapsulate the multifaceted nature of AD, which includes various disease trajectories. To assign axes to data trajectories without explicit ground truth labels, we implement a longitudinal contrastive loss leveraging self-supervision, thereby refining the separation of disease trajectories. Evaluated on the Alzheimer's Disease Neuroimaging Initiative (ADNI) dataset (N=1321), our model demonstrates superior performance in delineating between cognitively normal (CN), mild cognitive impairment (MCI), and AD subjects,classification of stable MCI vs converting MCI and Amyloid status, compared to the single-axis model. This is further substantiated through an  ablation study on the contrastive loss, underscoring the utility of our multi-axis approach in capturing the complex progression patterns of AD. The code is available at: https://github.com/jianweizhang17/MultiAxisDisentanglement.git",
  "authors": [
    {
      "name": "Zhang, Jianwei",
      "affiliation": null,
      "email": null
    },
    {
      "name": "Shi, Yonggang",
      "affiliation": null,
      "email": null
    }
  ],
  "subject_areas": [
    "Body -> Brain",
    "Modalities -> MRI",
    "Applications -> Computational (Integrative) Pathology",
    "Applications -> Computer Aided Diagnosis",
    "Machine Learning -> Deep Learning",
    "Machine Learning -> Interpretability / Explainability"
  ],
  "external_links": [
    {
      "type": "pdf",
      "url": "https://papers.miccai.org/miccai-2025/paper/0002_paper.pdf",
      "description": "Full paper PDF"
    }
  ],
  "publication_date": "2025-10-01",
  "raw_data_source": "{}"
}"#;

    #[test]
    fn test_author_serialization() {
        let author = Author {
            name: "John Doe".to_string(),
            affiliation: Some("University".to_string()),
            email: Some("john@example.com".to_string()),
        };

        let json = serde_json::to_string(&author).unwrap();
        assert!(json.contains(r#""name":"John Doe""#));
        assert!(json.contains(r#""affiliation":"University""#));
        assert!(json.contains(r#""email":"john@example.com""#));
    }

    #[test]
    fn test_author_with_none_fields() {
        let author = Author {
            name: "Jane Doe".to_string(),
            affiliation: None,
            email: None,
        };

        let json = serde_json::to_string(&author).unwrap();
        // None fields should not appear in JSON
        assert!(!json.contains("affiliation"));
        assert!(!json.contains("email"));
        assert!(json.contains(r#""name":"Jane Doe""#));
    }

    #[test]
    fn test_external_link_serialization() {
        let link = ExternalLink {
            link_type: "pdf".to_string(),
            url: "https://example.com/paper.pdf".to_string(),
            description: Some("Full paper".to_string()),
        };

        let json = serde_json::to_string(&link).unwrap();
        // Verify "type" field is used (not "link_type")
        assert!(json.contains(r#""type":"pdf""#));
        assert!(json.contains(r#""url":"https://example.com/paper.pdf""#));
        assert!(json.contains(r#""description":"Full paper""#));
    }

    #[test]
    fn test_paper_deserialization() {
        // Test deserialization from sample JSON
        let paper: Paper = serde_json::from_str(SAMPLE_PAPER_JSON).unwrap();

        assert_eq!(paper.id, "miccai-0002");
        assert_eq!(
            paper.title,
            "Surface-based Multi-Axis Longitudinal Disentanglement Using Contrastive Learning for Alzheimer's Disease"
        );
        assert_eq!(paper.authors.len(), 2);
        assert_eq!(paper.authors[0].name, "Zhang, Jianwei");
        assert_eq!(paper.authors[1].name, "Shi, Yonggang");
        assert_eq!(paper.subject_areas.len(), 6);
        assert_eq!(paper.external_links.len(), 1);
        assert_eq!(paper.external_links[0].link_type, "pdf");
        assert_eq!(paper.publication_date, Some("2025-10-01".to_string()));
    }

    #[test]
    fn test_paper_serialization() {
        let paper = Paper {
            id: "test-001".to_string(),
            title: "Test Paper".to_string(),
            abstract_text: "This is a test abstract.".to_string(),
            authors: vec![Author {
                name: "Test Author".to_string(),
                affiliation: None,
                email: None,
            }],
            subject_areas: vec!["Machine Learning".to_string()],
            external_links: vec![],
            publication_date: Some("2025-01-01".to_string()),
            raw_data_source: None,
        };

        let json = serde_json::to_string(&paper).unwrap();
        // Verify "abstract" field name (not "abstract_text")
        assert!(json.contains(r#""abstract":"This is a test abstract.""#));
        assert!(json.contains(r#""id":"test-001""#));
        assert!(json.contains(r#""title":"Test Paper""#));
    }

    #[test]
    fn test_paper_round_trip() {
        // Deserialize from sample JSON
        let paper: Paper = serde_json::from_str(SAMPLE_PAPER_JSON).unwrap();

        // Serialize back to JSON
        let serialized = serde_json::to_string_pretty(&paper).unwrap();

        // Deserialize again
        let paper2: Paper = serde_json::from_str(&serialized).unwrap();

        // Should be equal
        assert_eq!(paper, paper2);
    }

    #[test]
    fn test_paper_similarity_serialization() {
        let paper = Paper {
            id: "test-001".to_string(),
            title: "Test Paper".to_string(),
            abstract_text: "Abstract".to_string(),
            authors: vec![],
            subject_areas: vec![],
            external_links: vec![],
            publication_date: None,
            raw_data_source: None,
        };

        let similarity = PaperSimilarity {
            paper_id: "test-001".to_string(),
            similarity_score: 0.95,
            paper,
        };

        let json = serde_json::to_string(&similarity).unwrap();
        assert!(json.contains(r#""paper_id":"test-001""#));
        assert!(json.contains(r#""similarity_score":0.95"#));
    }

    #[test]
    fn test_graph_node_serialization() {
        let node = GraphNode {
            id: "miccai-0001".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
            subject_areas: vec!["ML".to_string()],
            x: Some(1.5),
            y: Some(2.5),
            cluster: Some(0),
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""id":"miccai-0001""#));
        assert!(json.contains(r#""x":1.5"#));
        assert!(json.contains(r#""y":2.5"#));
        assert!(json.contains(r#""cluster":0"#));
    }

    #[test]
    fn test_graph_node_without_coordinates() {
        let node = GraphNode {
            id: "miccai-0001".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["Author 1".to_string()],
            subject_areas: vec![],
            x: None,
            y: None,
            cluster: None,
        };

        let json = serde_json::to_string(&node).unwrap();
        // None fields should not appear
        assert!(!json.contains(r#""x""#));
        assert!(!json.contains(r#""y""#));
        assert!(!json.contains(r#""cluster""#));
    }

    #[test]
    fn test_graph_edge_serialization() {
        let edge = GraphEdge {
            source: "miccai-0001".to_string(),
            target: "miccai-0002".to_string(),
            similarity: 0.85,
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains(r#""source":"miccai-0001""#));
        assert!(json.contains(r#""target":"miccai-0002""#));
        assert!(json.contains(r#""similarity":0.85"#));
    }

    #[test]
    fn test_graph_data_serialization() {
        let node1 = GraphNode {
            id: "miccai-0001".to_string(),
            title: "Paper 1".to_string(),
            authors: vec!["Author 1".to_string()],
            subject_areas: vec![],
            x: Some(0.0),
            y: Some(0.0),
            cluster: Some(0),
        };

        let node2 = GraphNode {
            id: "miccai-0002".to_string(),
            title: "Paper 2".to_string(),
            authors: vec!["Author 2".to_string()],
            subject_areas: vec![],
            x: Some(1.0),
            y: Some(1.0),
            cluster: Some(0),
        };

        let edge = GraphEdge {
            source: "miccai-0001".to_string(),
            target: "miccai-0002".to_string(),
            similarity: 0.9,
        };

        let graph_data = GraphData {
            nodes: vec![node1, node2],
            edges: vec![edge],
            clusters: None,
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        assert!(json.contains(r#""nodes":"#));
        assert!(json.contains(r#""edges":"#));
        // clusters is None, should not appear
        assert!(!json.contains(r#""clusters""#));
    }

    #[test]
    fn test_graph_data_with_clusters() {
        let mut clusters = HashMap::new();
        clusters.insert(
            "0".to_string(),
            serde_json::json!({
                "label": "Machine Learning",
                "count": 10
            }),
        );

        let graph_data = GraphData {
            nodes: vec![],
            edges: vec![],
            clusters: Some(clusters),
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        assert!(json.contains(r#""clusters":"#));
        assert!(json.contains(r#""Machine Learning""#));
    }

    #[test]
    fn test_paper_with_empty_collections() {
        let paper = Paper {
            id: "test-001".to_string(),
            title: "Test".to_string(),
            abstract_text: "Abstract".to_string(),
            authors: vec![],
            subject_areas: vec![],
            external_links: vec![],
            publication_date: None,
            raw_data_source: None,
        };

        let json = serde_json::to_string(&paper).unwrap();
        // Empty vecs should serialize to empty arrays
        assert!(json.contains(r#""authors":[]"#));
        assert!(json.contains(r#""subject_areas":[]"#));
        assert!(json.contains(r#""external_links":[]"#));
    }

    #[test]
    fn test_field_naming_conventions() {
        // Verify that field names use snake_case in JSON
        let paper = Paper {
            id: "test".to_string(),
            title: "Test".to_string(),
            abstract_text: "Abstract".to_string(),
            authors: vec![],
            subject_areas: vec!["ML".to_string()],
            external_links: vec![],
            publication_date: Some("2025-01-01".to_string()),
            raw_data_source: Some("{}".to_string()),
        };

        let json = serde_json::to_string(&paper).unwrap();
        // Verify snake_case field names
        assert!(json.contains(r#""subject_areas""#));
        assert!(json.contains(r#""external_links""#));
        assert!(json.contains(r#""publication_date""#));
        assert!(json.contains(r#""raw_data_source""#));

        let similarity = PaperSimilarity {
            paper_id: "test".to_string(),
            similarity_score: 0.5,
            paper,
        };

        let json = serde_json::to_string(&similarity).unwrap();
        assert!(json.contains(r#""paper_id""#));
        assert!(json.contains(r#""similarity_score""#));
    }
}
