use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the graph visualization (a paper).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub subject_areas: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
}

/// Represents an edge between two papers in the graph (similarity connection).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub similarity: f64,
}

/// Represents the complete graph data structure with nodes, edges, and optional cluster information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_serialization() {
        let node = GraphNode {
            id: "paper_001".to_string(),
            title: "Example Paper".to_string(),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            subject_areas: vec!["Machine Learning".to_string()],
            x: Some(10.5),
            y: Some(-5.3),
            cluster: Some(0),
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"id\":\"paper_001\""));
        assert!(json.contains("\"title\":\"Example Paper\""));
        assert!(json.contains("\"x\":10.5"));
        assert!(json.contains("\"y\":-5.3"));
        assert!(json.contains("\"cluster\":0"));
    }

    #[test]
    fn test_graph_node_without_coordinates() {
        let node = GraphNode {
            id: "paper_002".to_string(),
            title: "Another Paper".to_string(),
            authors: vec![],
            subject_areas: vec![],
            x: None,
            y: None,
            cluster: None,
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"id\":\"paper_002\""));
        assert!(!json.contains("\"x\""));
        assert!(!json.contains("\"y\""));
        assert!(!json.contains("\"cluster\""));
    }

    #[test]
    fn test_graph_node_deserialization() {
        let json = r#"{
            "id": "paper_001",
            "title": "Test Paper",
            "authors": ["Author One"],
            "subject_areas": ["AI"],
            "x": 1.5,
            "y": 2.5,
            "cluster": 1
        }"#;

        let node: GraphNode = serde_json::from_str(json).unwrap();
        assert_eq!(node.id, "paper_001");
        assert_eq!(node.title, "Test Paper");
        assert_eq!(node.authors.len(), 1);
        assert_eq!(node.x, Some(1.5));
        assert_eq!(node.y, Some(2.5));
        assert_eq!(node.cluster, Some(1));
    }

    #[test]
    fn test_graph_edge_serialization() {
        let edge = GraphEdge {
            source: "paper_001".to_string(),
            target: "paper_002".to_string(),
            similarity: 0.85,
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("\"source\":\"paper_001\""));
        assert!(json.contains("\"target\":\"paper_002\""));
        assert!(json.contains("\"similarity\":0.85"));
    }

    #[test]
    fn test_graph_edge_deserialization() {
        let json = r#"{
            "source": "paper_001",
            "target": "paper_002",
            "similarity": 0.75
        }"#;

        let edge: GraphEdge = serde_json::from_str(json).unwrap();
        assert_eq!(edge.source, "paper_001");
        assert_eq!(edge.target, "paper_002");
        assert_eq!(edge.similarity, 0.75);
    }

    #[test]
    fn test_graph_data_serialization() {
        let node1 = GraphNode {
            id: "paper_001".to_string(),
            title: "Paper 1".to_string(),
            authors: vec![],
            subject_areas: vec![],
            x: Some(0.0),
            y: Some(0.0),
            cluster: Some(0),
        };

        let node2 = GraphNode {
            id: "paper_002".to_string(),
            title: "Paper 2".to_string(),
            authors: vec![],
            subject_areas: vec![],
            x: Some(1.0),
            y: Some(1.0),
            cluster: Some(0),
        };

        let edge = GraphEdge {
            source: "paper_001".to_string(),
            target: "paper_002".to_string(),
            similarity: 0.9,
        };

        let graph_data = GraphData {
            nodes: vec![node1, node2],
            edges: vec![edge],
            clusters: None,
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"edges\""));
        assert!(!json.contains("\"clusters\""));
    }

    #[test]
    fn test_graph_data_with_clusters() {
        let node = GraphNode {
            id: "paper_001".to_string(),
            title: "Paper 1".to_string(),
            authors: vec![],
            subject_areas: vec![],
            x: None,
            y: None,
            cluster: Some(0),
        };

        let mut clusters = HashMap::new();
        clusters.insert("0".to_string(), serde_json::json!({"label": "Cluster 0"}));

        let graph_data = GraphData {
            nodes: vec![node],
            edges: vec![],
            clusters: Some(clusters),
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        assert!(json.contains("\"clusters\""));
    }

    #[test]
    fn test_graph_data_deserialization() {
        let json = r#"{
            "nodes": [
                {
                    "id": "paper_001",
                    "title": "Paper 1",
                    "authors": [],
                    "subject_areas": []
                }
            ],
            "edges": [
                {
                    "source": "paper_001",
                    "target": "paper_002",
                    "similarity": 0.8
                }
            ]
        }"#;

        let graph_data: GraphData = serde_json::from_str(json).unwrap();
        assert_eq!(graph_data.nodes.len(), 1);
        assert_eq!(graph_data.edges.len(), 1);
        assert_eq!(graph_data.clusters, None);
    }

    #[test]
    fn test_graph_data_empty() {
        let graph_data = GraphData {
            nodes: vec![],
            edges: vec![],
            clusters: None,
        };

        let json = serde_json::to_string(&graph_data).unwrap();
        let deserialized: GraphData = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.nodes.len(), 0);
        assert_eq!(deserialized.edges.len(), 0);
    }
}
