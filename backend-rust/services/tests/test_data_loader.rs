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

// ==================== Caching Tests ====================

#[test]
fn test_paper_caching() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Load the index to get a valid paper ID
    let index = loader.load_paper_index().expect("Failed to load index");
    if index.papers.is_empty() {
        eprintln!("Skipping test: no papers in index");
        return;
    }

    let paper_id = &index.papers[0].id;

    // Get initial cache stats
    let (initial_hits, initial_misses) = loader.paper_cache_stats();

    // First load - should hit file system (cache miss)
    let paper1 = loader.get_paper_by_id(paper_id)
        .expect("Failed to load paper first time")
        .expect("Paper should exist");

    // Verify cache miss was recorded
    let (hits_after_first, misses_after_first) = loader.paper_cache_stats();
    assert_eq!(misses_after_first, initial_misses + 1, "First load should be a cache miss");
    assert_eq!(hits_after_first, initial_hits, "First load should not be a cache hit");

    // Second load - should hit cache
    let paper2 = loader.get_paper_by_id(paper_id)
        .expect("Failed to load paper second time")
        .expect("Paper should exist on second load");

    // Verify cache hit was recorded
    let (hits_after_second, misses_after_second) = loader.paper_cache_stats();
    assert_eq!(hits_after_second, initial_hits + 1, "Second load should be a cache hit");
    assert_eq!(misses_after_second, initial_misses + 1, "Second load should not add a cache miss");

    // Both loads should return the same data
    assert_eq!(paper1.id, paper2.id);
    assert_eq!(paper1.title, paper2.title);
    assert_eq!(paper1.abstract_text, paper2.abstract_text);
    assert_eq!(paper1.authors.len(), paper2.authors.len());

    println!("Successfully loaded paper '{}' from cache (1 miss, 1 hit verified)", paper_id);
}

#[test]
fn test_embedding_caching() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    // Try a known paper ID
    let paper_id = "miccai-0002";

    // Get initial cache stats
    let (initial_hits, initial_misses) = loader.embedding_cache_stats();

    // First load - should hit file system (cache miss)
    let embedding1_opt = loader.get_embedding_by_id(paper_id)
        .expect("Failed to load embedding first time");

    if embedding1_opt.is_none() {
        eprintln!("Skipping test: paper {} does not have an embedding", paper_id);
        return;
    }

    let embedding1 = embedding1_opt.unwrap();

    // Verify cache miss was recorded
    let (hits_after_first, misses_after_first) = loader.embedding_cache_stats();
    assert_eq!(misses_after_first, initial_misses + 1, "First load should be a cache miss");
    assert_eq!(hits_after_first, initial_hits, "First load should not be a cache hit");

    // Second load - should hit cache
    let embedding2 = loader.get_embedding_by_id(paper_id)
        .expect("Failed to load embedding second time")
        .expect("Embedding should exist on second load");

    // Verify cache hit was recorded
    let (hits_after_second, misses_after_second) = loader.embedding_cache_stats();
    assert_eq!(hits_after_second, initial_hits + 1, "Second load should be a cache hit");
    assert_eq!(misses_after_second, initial_misses + 1, "Second load should not add a cache miss");

    // Both loads should return the same data
    assert_eq!(embedding1.len(), embedding2.len());
    assert_eq!(embedding1, embedding2);

    println!("Successfully loaded embedding for '{}' from cache (1 miss, 1 hit verified)", paper_id);
}

#[test]
fn test_paper_index_caching() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // First load - should hit file system
    let index1 = loader.load_paper_index()
        .expect("Failed to load index first time");

    // Second load - should hit cache
    let index2 = loader.load_paper_index()
        .expect("Failed to load index second time");

    // Both loads should return the same data
    assert_eq!(index1.dataset_info.total_papers, index2.dataset_info.total_papers);
    assert_eq!(index1.dataset_info.generated_at, index2.dataset_info.generated_at);
    assert_eq!(index1.papers.len(), index2.papers.len());

    println!("Successfully loaded paper index from cache");
}

#[test]
fn test_get_all_papers_uses_cache() {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // First call to get_all_papers - should populate cache
    let papers1 = loader.get_all_papers()
        .expect("Failed to load all papers first time");

    assert!(!papers1.is_empty(), "Should have loaded at least one paper");

    // Get a paper ID from the first result
    let paper_id = &papers1[0].id;

    // Now directly call get_paper_by_id - should hit cache populated by get_all_papers
    let paper = loader.get_paper_by_id(paper_id)
        .expect("Failed to load paper by ID")
        .expect("Paper should exist in cache");

    // Should match the paper from get_all_papers
    assert_eq!(paper.id, papers1[0].id);
    assert_eq!(paper.title, papers1[0].title);

    println!("Verified cache is populated by get_all_papers");
}

// ==================== Concurrent Access Tests ====================

#[test]
fn test_concurrent_paper_access() {
    use std::sync::Arc;
    use std::thread;

    let loader = Arc::new(DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR)));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Load the index to get valid paper IDs
    let index = loader.load_paper_index().expect("Failed to load index");
    if index.papers.len() < 2 {
        eprintln!("Skipping test: need at least 2 papers for concurrent access test");
        return;
    }

    let paper_id1 = index.papers[0].id.clone();
    let paper_id2 = if index.papers.len() > 1 {
        index.papers[1].id.clone()
    } else {
        index.papers[0].id.clone()
    };

    // Spawn multiple threads that access papers concurrently
    let mut handles = vec![];

    for i in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let pid1 = paper_id1.clone();
        let pid2 = paper_id2.clone();

        let handle = thread::spawn(move || {
            // Each thread loads both papers multiple times
            for _ in 0..5 {
                let paper1 = loader_clone.get_paper_by_id(&pid1)
                    .expect("Failed to load paper 1")
                    .expect("Paper 1 should exist");

                let paper2 = loader_clone.get_paper_by_id(&pid2)
                    .expect("Failed to load paper 2")
                    .expect("Paper 2 should exist");

                assert_eq!(paper1.id, pid1);
                assert_eq!(paper2.id, pid2);
            }
            println!("Thread {} completed successfully", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Concurrent paper access test passed");
}

#[test]
fn test_concurrent_embedding_access() {
    use std::sync::Arc;
    use std::thread;

    let loader = Arc::new(DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR)));

    // Skip test if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping test: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    // Try a known paper ID
    let paper_id = "miccai-0002";

    // Verify the embedding exists before running concurrent test
    let test_embedding = loader.get_embedding_by_id(paper_id)
        .expect("Failed to load test embedding");

    if test_embedding.is_none() {
        eprintln!("Skipping test: paper {} does not have an embedding", paper_id);
        return;
    }

    // Spawn multiple threads that access the same embedding concurrently
    let mut handles = vec![];

    for i in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let pid = paper_id.to_string();

        let handle = thread::spawn(move || {
            // Each thread loads the embedding multiple times
            for _ in 0..5 {
                let embedding = loader_clone.get_embedding_by_id(&pid)
                    .expect("Failed to load embedding")
                    .expect("Embedding should exist");

                assert!(!embedding.is_empty());
            }
            println!("Thread {} completed successfully", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Concurrent embedding access test passed");
}

#[test]
fn test_concurrent_mixed_access() {
    use std::sync::Arc;
    use std::thread;

    let loader = Arc::new(DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR)));

    // Skip test if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping test: data directory not found at {}", DATA_DIR);
        return;
    }

    // Load the index to get a valid paper ID
    let index = loader.load_paper_index().expect("Failed to load index");
    if index.papers.is_empty() {
        eprintln!("Skipping test: no papers in index");
        return;
    }

    let paper_id = index.papers[0].id.clone();

    // Spawn multiple threads that access papers, embeddings, and index concurrently
    let mut handles = vec![];

    for i in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let pid = paper_id.clone();

        let handle = thread::spawn(move || {
            for _ in 0..3 {
                // Load paper
                let _paper = loader_clone.get_paper_by_id(&pid)
                    .expect("Failed to load paper");

                // Load embedding (may not exist, that's OK)
                let _embedding = loader_clone.get_embedding_by_id(&pid)
                    .expect("Failed to attempt loading embedding");

                // Load index
                let _index = loader_clone.load_paper_index()
                    .expect("Failed to load index");
            }
            println!("Thread {} completed successfully", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Concurrent mixed access test passed");
}
