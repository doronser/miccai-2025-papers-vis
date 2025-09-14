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

## Recent Changes
- Created feature specification with functional requirements (FR-001 to FR-013)
- Completed research phase - selected D3.js, FastAPI, sentence-transformers
- Designed data model with Paper/Author/Link entities and embedding vectors
- Created OpenAPI specification with REST endpoints for papers, search, similarity
- Generated quickstart guide with development setup and user story validation

## Quick Commands
```bash
# Backend setup
cd backend && python -m venv venv && source venv/bin/activate && pip install -r requirements.txt

# Frontend setup  
cd frontend && npm install

# Data pipeline
python -m cli.data_fetcher --source miccai --output data/papers.json
python -m cli.embeddings_generator --input data/papers.json --output data/papers_with_embeddings.json

# Development servers
python -m uvicorn main:app --reload  # Backend
npm run dev                          # Frontend
```

## Testing Strategy
- Contract tests with OpenAPI validation
- Integration tests with real MICCAI data
- E2E tests for user scenarios (search, favorites, graph interaction)
- Performance validation for large datasets

For detailed implementation planning, see `/specs/001-plan-a-webapp/` directory.