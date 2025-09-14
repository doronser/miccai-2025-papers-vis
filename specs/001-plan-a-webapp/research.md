# Research Findings: MICCAI 2025 Papers Visualization

## Data Source Analysis

**Decision**: Use https://papers.miccai.org/miccai-2025/ as primary data source with web scraping fallback  
**Rationale**: Official conference website provides structured data, likely in JSON format or scrapeable HTML  
**Alternatives considered**: 
- Manual data entry (rejected: too time-consuming)
- Third-party academic APIs like Semantic Scholar (rejected: may not have all MICCAI 2025 papers yet)

## Graph Visualization Technology

**Decision**: Use D3.js for custom graph visualization with vis.js as fallback  
**Rationale**: D3.js provides maximum flexibility for custom interactions, clustering, and styling  
**Alternatives considered**:
- vis.js (simpler but less customizable)
- Cytoscape.js (good for biological networks but heavier)
- React Flow (designed for workflows, not academic paper networks)

## Embeddings and Similarity

**Decision**: Use sentence-transformers with all-MiniLM-L6-v2 model for paper abstract embeddings  
**Rationale**: Lightweight, fast, good semantic understanding for clustering related papers  
**Alternatives considered**:
- OpenAI API embeddings (rejected: cost and API dependency)
- TF-IDF with cosine similarity (rejected: less semantic understanding)
- SciBERT (rejected: larger model, slower inference)

## Backend Architecture

**Decision**: FastAPI with Python 3.11+ for REST API backend  
**Rationale**: Fast development, automatic OpenAPI docs, excellent async support for embeddings  
**Alternatives considered**:
- Flask (rejected: less modern, no built-in async)
- Django (rejected: overkill for this use case)
- Node.js (rejected: Python preferred for ML libraries)

## Frontend Framework

**Decision**: React 18 with TypeScript and Vite build system  
**Rationale**: Modern React features, excellent TypeScript support, fast development with Vite  
**Alternatives considered**:
- Next.js (rejected: SSR not needed for this use case)
- Vue.js (rejected: less ecosystem for data visualization)
- Vanilla JavaScript with D3 (rejected: harder state management)

## Data Storage Strategy

**Decision**: Static JSON files initially, with database migration path planned  
**Rationale**: Simple deployment, no DB setup required, easy to version control paper data  
**Alternatives considered**:
- PostgreSQL immediately (rejected: adds complexity for initial version)
- SQLite (rejected: harder to deploy and share)
- In-memory only (rejected: slow startup, no persistence)

## Deployment Strategy

**Decision**: Static site generation with API deployment to Vercel/Netlify  
**Rationale**: Easy sharing, fast CDN delivery, simple CI/CD  
**Alternatives considered**:
- Docker containers (planned for future auth version)
- Traditional VPS hosting (rejected: more maintenance)
- GitHub Pages (rejected: no backend support)

## User Authentication (Future)

**Decision**: JWT tokens with optional anonymous usage for initial version  
**Rationale**: Allows gradual migration from anonymous to authenticated usage  
**Alternatives considered**:
- OAuth only (rejected: barriers to entry)
- No auth ever (rejected: can't save favorites persistently)
- Session-based (rejected: harder for API usage)

## Search and Filtering

**Decision**: Client-side search with backend pre-processing for performance  
**Rationale**: Fast interactions, works offline once loaded, reduces server load  
**Alternatives considered**:
- Server-side search only (rejected: slower user experience)
- Full-text search engine like Elasticsearch (rejected: overkill for 1000 papers)

## Testing Strategy

**Decision**: Contract tests with OpenAPI validation, integration tests with real data  
**Rationale**: Ensures API contracts are maintained, catches real-world data issues  
**Alternatives considered**:
- Unit tests only (rejected: misses integration issues)
- E2E tests only (rejected: slow feedback, brittle)
- Mock data testing (rejected: misses real data edge cases)