# MICCAI 2025 Papers Data

This directory contains scraped and processed MICCAI 2025 papers data for the visualization webapp.

## Data Overview

- **Total Papers**: 1,008 papers (99.3% coverage of MICCAI 2025)
- **Data Format**: JSON files with NPZ embeddings
- **Storage**: ~2MB per paper, ~2.8KB per embedding
- **Quality**: 100% PDF availability, validated metadata

## Data Files

- `papers_by_id/` - Individual paper JSON files (1,008 files)
- `embeddings_by_id/` - Semantic embeddings for each paper (1,008 NPZ files)
- `embeddings_by_id__simple/` - Simplified embeddings (backup/alternative)
- `cache/` - Cached network data and t-SNE coordinates
- `dataset_stats.json` - Comprehensive dataset statistics

## Data Structure

Each paper contains:
- **Basic info**: title, abstract, authors, publication date
- **Topics**: Detailed subject areas (e.g., "Brain", "MRI", "Machine Learning")
- **Links**: PDF URLs and external resources
- **Embeddings**: 384-dimensional vectors for similarity search

## Features

- **Parallel scraping** with rate limiting
- **Detailed topic extraction** from individual paper pages
- **Real abstracts** extracted from paper pages
- **Semantic embeddings** using sentence-transformers/all-MiniLM-L6-v2
- **Automatic backups** before data updates
- **Cached computations** for performance optimization

## Usage

The data is automatically loaded by the backend API. No manual intervention required for normal operation.

### For Development/Research

If you need to regenerate the data:

```bash
# Scrape papers (run once)
python -m src.lib.miccai_parallel_scraper --max-workers 8

# Generate embeddings (run once)
python -m src.lib.scibert_embeddings
```

### Data Statistics

See `dataset_stats.json` for detailed metrics including:
- Author distribution
- Content length statistics
- Subject area distribution
- PDF availability rates
