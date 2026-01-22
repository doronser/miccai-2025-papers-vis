use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Author information for a paper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// External link associated with a paper (e.g., PDF, website)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalLink {
    #[serde(rename = "type")]
    pub link_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Core paper metadata
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data_source: Option<String>,
}

/// Paper with similarity score for similarity search results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaperSimilarity {
    pub paper_id: String,
    pub similarity_score: f32,
    pub paper: Paper,
}

/// Node in the graph visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub subject_areas: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
}

/// Edge in the graph visualization connecting two papers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub similarity: f32,
}

/// Complete graph data for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_deserialization_from_json() {
        let json_str = r#"{
            "id": "miccai-1274",
            "title": "VMRA-MaR: An Asymmetry-Aware Temporal Framework for Longitudinal Breast Cancer Risk Prediction",
            "abstract": "Breast cancer remains a leading cause of mortality worldwide and is typically detected via screening programs where healthy people are invited in regular intervals.",
            "authors": [
                {
                    "name": "Sun, Zijun",
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

        let paper: Paper = serde_json::from_str(json_str).expect("Failed to deserialize paper");

        assert_eq!(paper.id, "miccai-1274");
        assert_eq!(paper.title, "VMRA-MaR: An Asymmetry-Aware Temporal Framework for Longitudinal Breast Cancer Risk Prediction");
        assert!(paper.abstract_text.contains("Breast cancer remains"));
        assert_eq!(paper.authors.len(), 1);
        assert_eq!(paper.authors[0].name, "Sun, Zijun");
        assert_eq!(paper.subject_areas.len(), 2);
        assert_eq!(paper.external_links.len(), 1);
        assert_eq!(paper.external_links[0].link_type, "pdf");
    }

    #[test]
    fn test_paper_serialization_to_json() {
        let paper = Paper {
            id: "test-123".to_string(),
            title: "Test Paper".to_string(),
            abstract_text: "This is a test abstract.".to_string(),
            authors: vec![Author {
                name: "Jane Doe".to_string(),
                affiliation: Some("University of Test".to_string()),
                email: None,
            }],
            subject_areas: vec!["AI".to_string(), "ML".to_string()],
            external_links: vec![ExternalLink {
                link_type: "pdf".to_string(),
                url: "https://example.com/paper.pdf".to_string(),
                description: Some("Paper PDF".to_string()),
            }],
            publication_date: Some("2025-01-01".to_string()),
            raw_data_source: None,
        };

        let json_str = serde_json::to_string(&paper).expect("Failed to serialize paper");
        let json_value: serde_json::Value = serde_json::from_str(&json_str).expect("Invalid JSON");

        // Verify field names in JSON output
        assert_eq!(json_value["id"], "test-123");
        assert_eq!(json_value["title"], "Test Paper");
        assert_eq!(json_value["abstract"], "This is a test abstract.");
        assert!(json_value["authors"].is_array());
        assert_eq!(json_value["authors"][0]["name"], "Jane Doe");
        assert_eq!(json_value["external_links"][0]["type"], "pdf");
        // raw_data_source should not be present when None
        assert!(json_value.get("raw_data_source").is_none());
    }

    #[test]
    fn test_paper_with_empty_arrays() {
        let json_str = r#"{
            "id": "test-456",
            "title": "Empty Arrays Test",
            "abstract": "Testing empty arrays.",
            "authors": [],
            "subject_areas": [],
            "external_links": []
        }"#;

        let paper: Paper = serde_json::from_str(json_str).expect("Failed to deserialize paper");

        assert_eq!(paper.authors.len(), 0);
        assert_eq!(paper.subject_areas.len(), 0);
        assert_eq!(paper.external_links.len(), 0);
    }

    #[test]
    fn test_paper_similarity() {
        let paper = Paper {
            id: "test-789".to_string(),
            title: "Similar Paper".to_string(),
            abstract_text: "Abstract text.".to_string(),
            authors: vec![],
            subject_areas: vec![],
            external_links: vec![],
            publication_date: None,
            raw_data_source: None,
        };

        let similarity = PaperSimilarity {
            paper_id: "test-789".to_string(),
            similarity_score: 0.95,
            paper,
        };

        let json_str = serde_json::to_string(&similarity).expect("Failed to serialize");
        let deserialized: PaperSimilarity =
            serde_json::from_str(&json_str).expect("Failed to deserialize");

        assert_eq!(deserialized.paper_id, "test-789");
        assert!((deserialized.similarity_score - 0.95).abs() < 1e-6);
    }

    #[test]
    fn test_graph_node() {
        let node = GraphNode {
            id: "node-1".to_string(),
            title: "Node Title".to_string(),
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
            subject_areas: vec!["Area 1".to_string()],
            x: Some(10.5),
            y: Some(20.3),
            cluster: Some(1),
        };

        let json_str = serde_json::to_string(&node).expect("Failed to serialize");
        let deserialized: GraphNode =
            serde_json::from_str(&json_str).expect("Failed to deserialize");

        assert_eq!(deserialized.id, "node-1");
        assert_eq!(deserialized.x, Some(10.5));
        assert_eq!(deserialized.cluster, Some(1));
    }

    #[test]
    fn test_graph_edge() {
        let edge = GraphEdge {
            source: "node-1".to_string(),
            target: "node-2".to_string(),
            similarity: 0.85,
        };

        let json_str = serde_json::to_string(&edge).expect("Failed to serialize");
        let deserialized: GraphEdge =
            serde_json::from_str(&json_str).expect("Failed to deserialize");

        assert_eq!(deserialized.source, "node-1");
        assert_eq!(deserialized.target, "node-2");
        assert!((deserialized.similarity - 0.85).abs() < 1e-6);
    }

    #[test]
    fn test_graph_data() {
        let graph_data = GraphData {
            nodes: vec![GraphNode {
                id: "node-1".to_string(),
                title: "Title 1".to_string(),
                authors: vec![],
                subject_areas: vec![],
                x: None,
                y: None,
                cluster: None,
            }],
            edges: vec![],
            clusters: None,
        };

        let json_str = serde_json::to_string(&graph_data).expect("Failed to serialize");
        let deserialized: GraphData =
            serde_json::from_str(&json_str).expect("Failed to deserialize");

        assert_eq!(deserialized.nodes.len(), 1);
        assert_eq!(deserialized.edges.len(), 0);
    }

    #[test]
    fn test_graph_data_with_clusters() {
        // Create a HashMap for clusters
        let mut clusters = HashMap::new();
        clusters.insert(
            "cluster_0".to_string(),
            serde_json::json!({"count": 5, "color": "#ff0000"}),
        );
        clusters.insert(
            "cluster_1".to_string(),
            serde_json::json!({"count": 3, "color": "#00ff00"}),
        );

        let graph_data = GraphData {
            nodes: vec![],
            edges: vec![],
            clusters: Some(clusters),
        };

        // Serialize to JSON
        let json_str = serde_json::to_string(&graph_data).expect("Failed to serialize");
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Verify clusters is an object (not array, string, etc.)
        assert!(
            json_value["clusters"].is_object(),
            "clusters should be a JSON object"
        );
        assert_eq!(json_value["clusters"]["cluster_0"]["count"], 5);
        assert_eq!(json_value["clusters"]["cluster_1"]["color"], "#00ff00");

        // Deserialize back
        let deserialized: GraphData =
            serde_json::from_str(&json_str).expect("Failed to deserialize");
        assert!(deserialized.clusters.is_some());
        assert_eq!(deserialized.clusters.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_graph_data_rejects_non_dict_clusters() {
        // Test that array is rejected for clusters field
        let json_with_array = r#"{
            "nodes": [],
            "edges": [],
            "clusters": ["item1", "item2"]
        }"#;

        let result = serde_json::from_str::<GraphData>(json_with_array);
        assert!(result.is_err(), "Should reject array for clusters field");

        // Test that string is rejected for clusters field
        let json_with_string = r#"{
            "nodes": [],
            "edges": [],
            "clusters": "some string"
        }"#;

        let result = serde_json::from_str::<GraphData>(json_with_string);
        assert!(result.is_err(), "Should reject string for clusters field");
    }
}
