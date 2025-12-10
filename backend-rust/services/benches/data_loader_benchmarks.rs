use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use services::data_loader::DataLoader;
use std::path::PathBuf;
use std::time::Duration;

const DATA_DIR: &str = "/l2l/src/miccai-2025-papers-vis/backend/src/data/papers_by_id";
const EMBEDDINGS_DIR: &str = "/l2l/src/miccai-2025-papers-vis/backend/src/data/embeddings_by_id";

/// Benchmark: Cache hit performance for papers (should be sub-millisecond)
fn benchmark_paper_cache_hit(c: &mut Criterion) {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip benchmark if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping benchmark: data directory not found at {}", DATA_DIR);
        return;
    }

    // Get a valid paper ID
    let index = match loader.load_paper_index() {
        Ok(idx) => idx,
        Err(_) => {
            eprintln!("Skipping benchmark: failed to load index");
            return;
        }
    };

    if index.papers.is_empty() {
        eprintln!("Skipping benchmark: no papers in index");
        return;
    }

    let paper_id = index.papers[0].id.clone();

    // Warm up cache
    let _ = loader.get_paper_by_id(&paper_id);

    // Benchmark cache hit
    c.bench_function("paper_cache_hit", |b| {
        b.iter(|| {
            loader.get_paper_by_id(black_box(&paper_id))
        });
    });
}

/// Benchmark: Cache hit performance for embeddings (should be sub-millisecond)
fn benchmark_embedding_cache_hit(c: &mut Criterion) {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip benchmark if embeddings directory doesn't exist
    if !loader.embeddings_dir().exists() {
        eprintln!("Skipping benchmark: embeddings directory not found at {}", EMBEDDINGS_DIR);
        return;
    }

    let paper_id = "miccai-0002";

    // Verify embedding exists
    match loader.get_embedding_by_id(paper_id) {
        Ok(Some(_)) => {},
        _ => {
            eprintln!("Skipping benchmark: paper {} has no embedding", paper_id);
            return;
        }
    }

    // Warm up cache (already done by the check above)

    // Benchmark cache hit
    c.bench_function("embedding_cache_hit", |b| {
        b.iter(|| {
            loader.get_embedding_by_id(black_box(paper_id))
        });
    });
}

/// Benchmark: Compare first access (file I/O) vs second access (cache hit)
fn benchmark_cache_vs_file_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_vs_file_io");

    // Skip benchmark if data directory doesn't exist
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));
    if !loader.papers_dir().exists() {
        eprintln!("Skipping benchmark: data directory not found");
        return;
    }

    let index = match loader.load_paper_index() {
        Ok(idx) => idx,
        Err(_) => {
            eprintln!("Skipping benchmark: failed to load index");
            return;
        }
    };

    if index.papers.len() < 5 {
        eprintln!("Skipping benchmark: need at least 5 papers");
        return;
    }

    // Benchmark first access (cold - requires file I/O)
    group.bench_function(BenchmarkId::new("first_access", "paper"), |b| {
        let mut paper_index = 0;
        b.iter(|| {
            // Use a different paper each iteration to avoid cache hits
            let paper_id = &index.papers[paper_index % 5].id;
            paper_index += 1;

            // Create new loader to ensure cold cache
            let fresh_loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));
            fresh_loader.get_paper_by_id(black_box(paper_id))
        });
    });

    // Benchmark second access (warm - from cache)
    let paper_id = &index.papers[0].id;
    let _ = loader.get_paper_by_id(paper_id); // Warm up cache

    group.bench_function(BenchmarkId::new("cache_hit", "paper"), |b| {
        b.iter(|| {
            loader.get_paper_by_id(black_box(paper_id))
        });
    });

    group.finish();
}

/// Benchmark: Load all papers (should complete in <2 seconds for test dataset)
fn benchmark_load_all_papers(c: &mut Criterion) {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    // Skip benchmark if data directory doesn't exist
    if !loader.papers_dir().exists() {
        eprintln!("Skipping benchmark: data directory not found");
        return;
    }

    // Set a longer measurement time for this benchmark since it's slower
    let mut group = c.benchmark_group("load_all_papers");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    group.bench_function("first_load", |b| {
        b.iter(|| {
            // Create new loader for each iteration to measure fresh load
            let fresh_loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));
            fresh_loader.get_all_papers()
        });
    });

    group.finish();
}

/// Benchmark: Index caching performance
fn benchmark_index_cache(c: &mut Criterion) {
    let loader = DataLoader::new(PathBuf::from(DATA_DIR), PathBuf::from(EMBEDDINGS_DIR));

    if !loader.papers_dir().exists() {
        eprintln!("Skipping benchmark: data directory not found");
        return;
    }

    // Warm up cache
    let _ = loader.load_paper_index();

    c.bench_function("index_cache_hit", |b| {
        b.iter(|| {
            loader.load_paper_index()
        });
    });
}

criterion_group!(
    benches,
    benchmark_paper_cache_hit,
    benchmark_embedding_cache_hit,
    benchmark_cache_vs_file_io,
    benchmark_load_all_papers,
    benchmark_index_cache
);
criterion_main!(benches);
