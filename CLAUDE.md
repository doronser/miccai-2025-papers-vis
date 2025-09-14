# Claude Code Context

This repository implements a MICCAI 2025 Papers Visualization Webapp that allows researchers to explore conference papers through interactive graph visualization.

## Project Overview
- **Feature**: MICCAI 2025 Papers Visualization Webapp
- **Branch**: 001-plan-a-webapp
- **Tech Stack**: Python 3.11+ backend (FastAPI), React 18+ frontend (TypeScript)
- **Architecture**: Web application with backend/ + frontend/ structure
- **Purpose**: Help researchers explore MICCAI 2025 papers via interactive graph visualization, search/filter capabilities, and favorites management

## Technology Stack
- **Backend**: FastAPI, sentence-transformers (embeddings), pytest
- **Frontend**: React 18, TypeScript, D3.js/vis.js (graph viz), Vite
- **Storage**: Static JSON files (extensible to PostgreSQL)
- **Deployment**: Vercel/Netlify for easy sharing

## Key Libraries and Dependencies
- **paper-processor**: Data extraction from miccai.org
- **visualization-engine**: Graph generation and clustering  
- **search-service**: Text search and filtering functionality
- **sentence-transformers**: all-MiniLM-L6-v2 for paper embeddings
- **D3.js**: Custom graph visualization with clustering

## Project Structure
```
backend/
├── src/
│   ├── models/          # Paper, Author, Link entities
│   ├── services/        # Data fetching, embeddings, search
│   └── api/             # FastAPI endpoints
└── tests/

frontend/
├── src/
│   ├── components/      # React components for graph, search, details
│   ├── pages/          # Main app pages
│   └── services/       # API client, data fetching
└── tests/
```

## Development Guidelines
- **Testing**: TDD mandatory - tests before implementation
- **Architecture**: Every feature as standalone library with CLI interface
- **Data Flow**: MICCAI Website → Embeddings → Graph Visualization → User Interaction
- **Performance Goals**: <2s load time, <500ms interactions, handle 1000 papers smoothly

## Current Status: Data Acquisition Phase ✅ COMPLETE

### Phase 1: Data Pipeline (COMPLETED)
- ✅ **MICCAI Website Scraped**: 1,007/1,014 papers (99.3% coverage)
- ✅ **Structured Data Extraction**: Title, abstract, authors, PDF links
- ✅ **Semantic Embeddings**: 1,007 × 384-dimensional vectors (sentence-transformers/all-MiniLM-L6-v2)
- ✅ **Static Data Storage**: JSON files by paper ID + NPZ embeddings
- ✅ **Quality Verification**: 100% PDF availability, 4,903 unique authors

### Implemented Libraries
- `src/lib/miccai_parallel_scraper.py` - High-performance MICCAI website scraper
- `src/lib/paper_parser.py` - HTML to structured Paper/Author entities
- `src/lib/scibert_embeddings.py` - Semantic embedding generation with CLI
- `src/data/papers_by_id/` - 1,007 individual paper JSON files
- `src/data/embeddings_by_id/` - 1,007 NPZ embedding files + statistics

### Performance Metrics
- **Scraping Speed**: ~31.77 papers/second (parallel processing)
- **Embedding Generation**: ~2 minutes for all 1,007 papers
- **Storage Efficiency**: ~2.8KB per embedding, ~2MB per paper
- **Data Quality**: 100% success rate, validated PDF links

## Next Phase: Backend API Development
Ready to implement FastAPI endpoints for paper search, similarity, and graph generation using the pre-computed embeddings.

## Quick Commands (Updated)
```bash
# Environment setup
python -m venv venv && source venv/bin/activate && pip install -r requirements.txt

# Data pipeline (COMPLETED - run once)
PYTHONPATH=. python -m src.lib.miccai_parallel_scraper  # Scrape papers
PYTHONPATH=. python -m src.lib.scibert_embeddings      # Generate embeddings

# Verify data
ls -1 src/data/papers_by_id/*.json | wc -l    # Should show 1008 (1007 + index)
ls -1 src/data/embeddings_by_id/*.npz | wc -l # Should show 1008 (1007 + stats)
```

## Testing Strategy
- Contract tests with OpenAPI validation
- Integration tests with real MICCAI data
- E2E tests for user scenarios (search, favorites, graph interaction)
- Performance validation for large datasets

For detailed implementation planning, see `/specs/001-plan-a-webapp/` directory.