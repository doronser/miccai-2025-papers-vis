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
    pub x: Option<f32>,
    
    /// Y coordinate (for t-SNE or other layout, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
    
    /// Cluster assignment (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<usize>,
}

/// Graph edge representing similarity between papers
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphEdge {
    /// Source paper ID
    pub source: String,
    
    /// Target paper ID
    pub target: String,
    
    /// Similarity score
    pub similarity: f32,
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
            x: Some(1.0),
            y: Some(2.0),
            cluster: Some(0),
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
            similarity: 0.85,
        };
        
        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("paper-1"));
        assert!(json.contains("paper-2"));
        assert!(json.contains("0.85"));
    }
}
