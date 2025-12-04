//! Example demonstrating model usage and validation
//!
//! Run with: cargo run -p models --example validate_models

use models::{Author, ExternalLink, GraphData, GraphEdge, GraphNode, Paper, PaperSimilarity};

fn main() {
    println!("=== Domain Models Validation ===\n");

    // Create a sample paper
    let paper = Paper {
        id: "example-001".to_string(),
        title: "Example Medical Imaging Paper".to_string(),
        abstract_text: "This is an example abstract for demonstration purposes.".to_string(),
        authors: vec![
            Author {
                name: "Dr. Alice Smith".to_string(),
                affiliation: Some("Stanford University".to_string()),
                email: Some("alice@stanford.edu".to_string()),
            },
            Author {
                name: "Dr. Bob Johnson".to_string(),
                affiliation: Some("MIT".to_string()),
                email: None,
            },
        ],
        subject_areas: vec![
            "Medical Imaging".to_string(),
            "Deep Learning".to_string(),
        ],
        external_links: vec![ExternalLink {
            link_type: "pdf".to_string(),
            url: "https://example.com/paper.pdf".to_string(),
            description: Some("Full paper PDF".to_string()),
        }],
        publication_date: Some("2025-01-15".to_string()),
        raw_data_source: Some("MICCAI 2025".to_string()),
    };

    println!("✅ Created Paper: {}", paper.title);
    println!("   ID: {}", paper.id);
    println!("   Authors: {}", paper.authors.len());
    println!("   Subject Areas: {}", paper.subject_areas.len());

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&paper).expect("Serialization failed");
    println!("\n📄 Serialized JSON:\n{}\n", json);

    // Create a paper similarity
    let similarity = PaperSimilarity {
        paper_id: paper.id.clone(),
        similarity_score: 0.95,
        paper: paper.clone(),
    };

    println!("✅ Created PaperSimilarity with score: {}", similarity.similarity_score);

    // Create graph nodes
    let node1 = GraphNode {
        id: "paper-1".to_string(),
        title: "Paper 1".to_string(),
        authors: vec!["Author A".to_string()],
        subject_areas: vec!["AI".to_string()],
        x: Some(10.5),
        y: Some(20.3),
        cluster: Some(0),
    };

    let node2 = GraphNode {
        id: "paper-2".to_string(),
        title: "Paper 2".to_string(),
        authors: vec!["Author B".to_string()],
        subject_areas: vec!["ML".to_string()],
        x: Some(15.2),
        y: Some(25.8),
        cluster: Some(0),
    };

    println!("✅ Created GraphNode 1: {} at ({:?}, {:?})", node1.title, node1.x, node1.y);
    println!("✅ Created GraphNode 2: {} at ({:?}, {:?})", node2.title, node2.x, node2.y);

    // Create graph edge
    let edge = GraphEdge {
        source: node1.id.clone(),
        target: node2.id.clone(),
        similarity: 0.87,
    };

    println!("✅ Created GraphEdge: {} → {} (similarity: {})", edge.source, edge.target, edge.similarity);

    // Create complete graph data
    let graph_data = GraphData {
        nodes: vec![node1, node2],
        edges: vec![edge],
        clusters: None,
    };

    println!("✅ Created GraphData with {} nodes and {} edges",
             graph_data.nodes.len(),
             graph_data.edges.len());

    // Serialize graph data
    let graph_json = serde_json::to_string_pretty(&graph_data).expect("Graph serialization failed");
    println!("\n📊 Graph Data JSON:\n{}\n", graph_json);

    println!("=== ✅ All models validated successfully! ===");
}
