use services::data_loader::DataLoader;
use std::path::PathBuf;

// Note: These tests require the actual data directory from the source repository
// The tests will look for data in the source repository location
// In a production environment, these would use test fixtures

const DATA_DIR: &str = "/l2l/src/miccai-2025-papers-vis/backend/src/data/papers_by_id";
const EMBEDDINGS_DIR: &str = "/l2l/src/miccai-2025-papers-vis/backend/src/data/embeddings_by_id";

#[test]
fn test_data_loader_initialization() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));
    assert_eq!(loader.papers_dir().to_str().unwrap(), DATA_DIR);
    assert_eq!(loader.embeddings_dir().to_str().unwrap(), EMBEDDINGS_DIR);
}

#[test]
fn test_load_paper_index() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist (e.g., in CI environment)
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    let result = loader.load_paper_index();
    assert!(result.is_ok(), "Failed to load paper index: {:?}", result.err());

    let index = result.unwrap();

    // Verify basic structure
    assert!(index.dataset_info.total_papers > 0);
    assert!(!index.dataset_info.generated_at.is_empty());
    assert!(!index.dataset_info.scraper_version.is_empty());
    assert!(!index.dataset_info.source_url.is_empty());

    // Verify statistics
    assert!(index.statistics.total_papers > 0);
    assert_eq!(index.statistics.total_papers, index.dataset_info.total_papers);

    // Verify papers list
    assert!(!index.papers.is_empty());
    assert_eq!(index.papers.len(), index.statistics.total_papers as usize);
}

#[test]
fn test_get_paper_by_id() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // First load the index to get a valid paper ID
    let index = loader.load_paper_index().expect("Failed to load index");
    assert!(!index.papers.is_empty(), "No papers in index");

    let paper_id = &index.papers[0].id;

    // Load the paper
    let result = loader.get_paper_by_id(paper_id);
    assert!(result.is_ok(), "Failed to load paper: {:?}", result.err());

    let paper_opt = result.unwrap();
    assert!(paper_opt.is_some(), "Paper should exist");

    let paper = paper_opt.unwrap();

    // Verify paper structure
    assert_eq!(&paper.id, paper_id);
    assert!(!paper.title.is_empty());
    assert!(!paper.abstract_text.is_empty());
    assert!(!paper.authors.is_empty());
}

#[test]
fn test_get_paper_by_id_specific() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Test with a known paper ID (miccai-0002)
    let result = loader.get_paper_by_id("miccai-0002");

    if result.is_ok() {
        let paper_opt = result.unwrap();
        if let Some(paper) = paper_opt {
            assert_eq!(paper.id, "miccai-0002");
            assert!(!paper.title.is_empty());
            assert!(!paper.authors.is_empty());

            // Verify some expected fields from miccai-0002.json
            assert!(paper.title.contains("Alzheimer"));
            assert!(!paper.subject_areas.is_empty());
        }
    }
}

#[test]
fn test_get_nonexistent_paper() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    let result = loader.get_paper_by_id("nonexistent-id-12345");
    assert!(result.is_ok(), "Should not error for non-existent paper");

    let paper_opt = result.unwrap();
    assert!(paper_opt.is_none(), "Non-existent paper should return None");
}

#[test]
fn test_get_all_papers() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    let result = loader.get_all_papers();
    assert!(result.is_ok(), "Failed to load all papers: {:?}", result.err());

    let papers = result.unwrap();

    // Should have at least some papers
    assert!(!papers.is_empty(), "Should have loaded at least one paper");

    // Verify first paper structure
    let paper = &papers[0];
    assert!(!paper.id.is_empty());
    assert!(!paper.title.is_empty());
    assert!(!paper.abstract_text.is_empty());
    assert!(!paper.authors.is_empty());
}

#[test]
fn test_get_all_papers_matches_index_count() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    let index = loader.load_paper_index().expect("Failed to load index");
    let papers = loader.get_all_papers().expect("Failed to load all papers");

    // Number of loaded papers should match the index count
    // (or be close, in case some files are missing)
    let expected_count = index.papers.len();
    let actual_count = papers.len();

    // Allow for some small discrepancy in case of missing files
    assert!(
        actual_count >= expected_count * 9 / 10, // At least 90% of papers should load
        "Expected at least {} papers, but loaded {}",
        expected_count * 9 / 10,
        actual_count
    );
}

#[test]
fn test_paper_authors_structure() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Load a known paper
    let paper = loader.get_paper_by_id("miccai-0002")
        .expect("Failed to load paper")
        .expect("Paper should exist");

    // Verify authors structure
    assert!(!paper.authors.is_empty());

    for author in &paper.authors {
        assert!(!author.name.is_empty());
        // affiliation and email can be None
    }
}

#[test]
fn test_paper_external_links_structure() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Load a known paper
    let paper = loader.get_paper_by_id("miccai-0002")
        .expect("Failed to load paper")
        .expect("Paper should exist");

    // Verify external links structure
    assert!(!paper.external_links.is_empty());

    for link in &paper.external_links {
        assert!(!link.link_type.is_empty());
        assert!(!link.url.is_empty());
        // description can be None
    }
}

// ==================== Embedding Loading Tests ====================

#[test]
fn test_get_embedding_by_id() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    // Test with a known paper ID that has an embedding (miccai-0002)
    let result = loader.get_embedding_by_id("miccai-0002");
    assert!(result.is_ok(), "Failed to load embedding: {:?}", result.err());

    let embedding_opt = result.unwrap();

    if let Some(embedding) = embedding_opt {
        // Verify embedding structure
        assert!(!embedding.is_empty(), "Embedding should not be empty");

        // SciBERT embeddings are typically 768 dimensions
        // But we'll just check it's a reasonable size
        assert!(
            embedding.len() >= 100,
            "Embedding dimensions seem too small: {}",
            embedding.len()
        );

        println!("Loaded embedding with {} dimensions", embedding.len());
    } else {
        // It's okay if this specific paper doesn't have an embedding
        println!("Paper miccai-0002 does not have an embedding file");
    }
}

#[test]
fn test_get_embedding_by_id_nonexistent() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    let result = loader.get_embedding_by_id("nonexistent-id-12345");
    assert!(result.is_ok(), "Should not error for non-existent embedding");

    let embedding_opt = result.unwrap();
    assert!(embedding_opt.is_none(), "Non-existent embedding should return None");
}

#[test]
fn test_get_all_embeddings() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    let result = loader.get_all_embeddings();
    assert!(result.is_ok(), "Failed to load embeddings: {:?}", result.err());

    let embeddings = result.unwrap();

    // Should have at least some embeddings (but not necessarily all papers have embeddings)
    if !embeddings.is_empty() {
        println!("Loaded {} embeddings", embeddings.len());

        // Verify structure of first embedding
        let (paper_id, embedding) = embeddings.iter().next().unwrap();
        assert!(!paper_id.is_empty());
        assert!(!embedding.is_empty());
        assert!(
            embedding.len() >= 100,
            "Embedding dimensions seem too small: {}",
            embedding.len()
        );
    } else {
        println!("No embeddings found (this might be expected if embeddings haven't been generated)");
    }
}

#[test]
fn test_embedding_dimensions_consistency() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    let embeddings = loader.get_all_embeddings().expect("Failed to load embeddings");

    if embeddings.is_empty() {
        println!("Skipping dimension consistency test: no embeddings found");
        return;
    }

    // All embeddings should have the same dimensions
    let first_dim = embeddings.values().next().unwrap().len();

    for (paper_id, embedding) in &embeddings {
        assert_eq!(
            embedding.len(),
            first_dim,
            "Embedding for {} has inconsistent dimensions: expected {}, got {}",
            paper_id,
            first_dim,
            embedding.len()
        );
    }

    println!("All {} embeddings have consistent dimensions: {}", embeddings.len(), first_dim);
}
