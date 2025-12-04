//! Graph data models for visualization
//!
//! Maps graph-related models from Python's `src/models/paper.py` to Rust.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Graph node representing a paper in the visualization
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphNode {
    /// Paper ID
    pub id: String,
    
    /// Paper title
    pub title: String,
    
    /// List of author names
    pub authors: Vec<String>,
    
    /// Subject areas
    pub subject_areas: Vec<String>,
    
    /// X coordinate (for t-SNE or other layout, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,

    /// Y coordinate (for t-SNE or other layout, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,

    /// Cluster assignment (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
}

/// Graph edge representing similarity between papers
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphEdge {
    /// Source paper ID
    pub source: String,

    /// Target paper ID
    pub target: String,

    /// Similarity score
    pub similarity: f64,
}

/// Complete graph data for visualization
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphData {
    /// List of graph nodes
    pub nodes: Vec<GraphNode>,
    
    /// List of graph edges
    pub edges: Vec<GraphEdge>,
    
    /// Optional cluster metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_serialization() {
        let node = GraphNode {
            id: "paper-1".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["Author 1".to_string()],
            subject_areas: vec!["AI".to_string()],
            x: Some(1.0_f64),
            y: Some(2.0_f64),
            cluster: Some(0_i32),
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("paper-1"));
        assert!(json.contains("Test Paper"));
    }

    #[test]
    fn test_graph_edge_serialization() {
        let edge = GraphEdge {
            source: "paper-1".to_string(),
            target: "paper-2".to_string(),
            similarity: 0.85_f64,
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("paper-1"));
        assert!(json.contains("paper-2"));
        assert!(json.contains("0.85"));
    }

    #[test]
    fn test_graph_data_with_clusters() {
        let graph_data = GraphData {
            nodes: vec![
                GraphNode {
                    id: "paper-1".to_string(),
                    title: "Test Paper 1".to_string(),
                    authors: vec!["Author A".to_string()],
                    subject_areas: vec!["AI".to_string()],
                    x: Some(1.0_f64),
                    y: Some(2.0_f64),
                    cluster: Some(0_i32),
                }
            ],
            edges: vec![
                GraphEdge {
                    source: "paper-1".to_string(),
                    target: "paper-2".to_string(),
                    similarity: 0.9_f64,
                }
            ],
            clusters: None,
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
        assert!(!json.contains("\"clusters\""));
    }

    #[test]
    fn test_graph_node_optional_fields() {
        let node = GraphNode {
            id: "paper-1".to_string(),
            title: "Test".to_string(),
            authors: vec![],
            subject_areas: vec![],
            x: None,
            y: None,
            cluster: None,
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(!json.contains("\"x\""));
        assert!(!json.contains("\"y\""));
        assert!(!json.contains("\"cluster\""));
    }

    #[test]
    fn test_graph_data_deserialization() {
        let json = r#"{
            "nodes": [
                {
                    "id": "paper-1",
                    "title": "Test Paper",
                    "authors": ["Author One"],
                    "subject_areas": ["ML"],
                    "x": 1.5,
                    "y": 2.5,
                    "cluster": 0
                }
            ],
            "edges": [
                {
                    "source": "paper-1",
                    "target": "paper-2",
                    "similarity": 0.85
                }
            ]
        }"#;

        let graph_data: GraphData = serde_json::from_str(json).unwrap();
        assert_eq!(graph_data.nodes.len(), 1);
        assert_eq!(graph_data.edges.len(), 1);
        assert_eq!(graph_data.nodes[0].id, "paper-1");
        assert_eq!(graph_data.edges[0].similarity, 0.85_f64);
    }

    #[test]
    fn test_graph_node_round_trip() {
        let original = GraphNode {
            id: "paper-123".to_string(),
            title: "Research Paper".to_string(),
            authors: vec!["Dr. Smith".to_string(), "Dr. Jones".to_string()],
            subject_areas: vec!["Computer Vision".to_string()],
            x: Some(10.5_f64),
            y: Some(20.3_f64),
            cluster: Some(3_i32),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: GraphNode = serde_json::from_str(&json).unwrap();

        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.title, deserialized.title);
        assert_eq!(original.authors, deserialized.authors);
        assert_eq!(original.x, deserialized.x);
        assert_eq!(original.y, deserialized.y);
        assert_eq!(original.cluster, deserialized.cluster);
    }
}
