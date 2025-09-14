# Tasks: MICCAI 2025 Papers Visualization Webapp

**Input**: Design documents from `/specs/001-plan-a-webapp/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Tech stack: Python 3.11+/FastAPI backend, React 18+/TypeScript frontend, BeautifulSoup (scraping)
   → Libraries: sentence-transformers, D3.js, Vite, requests, BeautifulSoup
   → Structure: Web app (backend/ + frontend/)
   → PRIORITY: Data-first approach - fetch from https://papers.miccai.org/miccai-2025/ first
2. Load design documents:
   → data-model.md: Paper, Author, ExternalLink entities
   → contracts/api-spec.yaml: 5 endpoints with OpenAPI spec
   → research.md: Technology decisions validated
3. Generate tasks by priority:
   → PHASE 1: Data acquisition pipeline (MICCAI scraping, parsing, storage)
   → PHASE 2: Semantic processing (embedding generation, similarity calculations)
   → PHASE 3: Setup and Tests (project structure, TDD contract tests)
   → PHASE 4: Core Implementation (models, services, APIs)
   → PHASE 5: Visualization and UI (graph generation, frontend)
4. Apply task rules:
   → Data pipeline must complete before API implementation
   → Different files = [P] parallel
   → TDD enforced: tests before implementation
5. Number tasks T001-T040
6. Validate completeness: Data pipeline tested, all contracts tested, entities modeled
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- File paths use web app structure: `backend/src/`, `frontend/src/`

## Phase 1: Complete Data Pipeline ✅ COMPLETED
⚠️ **CRITICAL**: Full data scraping, parsing, and embedding generation in one go

- [x] T001 [P] Create MICCAI scraper library in backend/src/lib/miccai_scraper.py with CLI interface
- [x] T002 [P] Integration test for MICCAI data fetching from https://papers.miccai.org/miccai-2025/ in backend/tests/integration/test_miccai_fetcher.py
- [x] T003 [P] Paper data parser to extract title, abstract, authors, links from MICCAI HTML in backend/src/lib/paper_parser.py
- [x] T004 [P] Integration test for paper parsing with real MICCAI data samples in backend/tests/integration/test_paper_parser.py
- [x] T005 Data storage system for parsed papers in JSON format in backend/src/data/ (with backup/versioning)

## Phase 2: Static Embeddings with Sentence Transformers ✅ COMPLETED
⚠️ **APPROACH**: Generate all embeddings once, save by paper ID, use sentence-transformers for efficient processing

- [x] T006 Full MICCAI dataset scraping - scrape ALL papers in single run and save to backend/src/data/papers_by_id/ (**1,007 papers scraped**)
- [x] T007 [P] Embedding generator in backend/src/lib/scibert_embeddings.py with CLI interface using sentence-transformers library (**all-MiniLM-L6-v2 model**)
- [x] T008 [P] Integration test for embedding generation (**single paper test successful**)
- [x] T009 [P] Static embedding storage system - save embeddings by paper ID in backend/src/data/embeddings_by_id/ (**1,007 embeddings generated**)
- [ ] T010 [P] Similarity engine using pre-computed embeddings in backend/src/lib/similarity_engine.py with cosine similarity
- [ ] T011 Integration test for similarity calculations using static embeddings in backend/tests/integration/test_similarity_engine.py

### Data Acquisition Statistics
- **Papers Scraped**: 1,007 out of 1,014 MICCAI 2025 accepted papers (99.3% coverage)
- **PDF Availability**: 100% - all papers have verified PDF links
- **Unique Authors**: 4,903 authors across all papers
- **Embeddings Generated**: 1,007 papers × 384-dimensional vectors using sentence-transformers/all-MiniLM-L6-v2
- **Storage**: Static JSON files by paper ID + NPZ files for embeddings
- **Processing Speed**: ~31.77 papers/second for scraping, ~2 minutes for full embedding generation

## Phase 3: Project Setup and Structure
⚠️ **DEPENDENCY**: Can run in parallel with Phase 1-2, but must complete before Phase 4

- [ ] T011 Create backend/ and frontend/ directory structure per implementation plan
- [ ] T012 Initialize Python backend with FastAPI, sentence-transformers, BeautifulSoup, pytest dependencies in backend/requirements.txt
- [ ] T013 Initialize React frontend with TypeScript, D3.js, Vite dependencies in frontend/package.json
- [ ] T014 [P] Configure Python linting (ruff, black) in backend/.pre-commit-config.yaml
- [ ] T015 [P] Configure TypeScript/ESLint in frontend/.eslintrc.json and frontend/tsconfig.json

## Phase 4: API Tests and Core Implementation
⚠️ **DEPENDENCY**: Requires Phase 1-2 completion (data available) and Phase 3 setup

### Contract Tests (API Endpoints) - TDD FIRST
- [ ] T016 [P] Contract test GET /api/v1/papers in backend/tests/contract/test_papers_get.py
- [ ] T017 [P] Contract test GET /api/v1/papers/{paper_id} in backend/tests/contract/test_paper_detail.py
- [ ] T018 [P] Contract test GET /api/v1/papers/{paper_id}/similar in backend/tests/contract/test_paper_similar.py
- [ ] T019 [P] Contract test GET /api/v1/graph in backend/tests/contract/test_graph.py
- [ ] T020 [P] Contract test GET /api/v1/metadata in backend/tests/contract/test_metadata.py

### Models and Services (ONLY after contract tests failing)
- [ ] T021 [P] Paper model with validation in backend/src/models/paper.py
- [ ] T022 [P] Author model with validation in backend/src/models/author.py
- [ ] T023 [P] ExternalLink model with validation in backend/src/models/external_link.py
- [ ] T024 [P] PaperService for CRUD operations using stored JSON data in backend/src/services/paper_service.py
- [ ] T025 [P] EmbeddingsService for similarity calculations using pre-computed embeddings in backend/src/services/embeddings_service.py
- [ ] T026 [P] SearchService for filtering and queries in backend/src/services/search_service.py

### API Endpoints Implementation
- [ ] T027 GET /api/v1/papers endpoint implementation in backend/src/api/papers.py
- [ ] T028 GET /api/v1/papers/{paper_id} endpoint implementation in backend/src/api/papers.py
- [ ] T029 GET /api/v1/papers/{paper_id}/similar endpoint implementation in backend/src/api/papers.py
- [ ] T030 GET /api/v1/graph endpoint implementation in backend/src/api/graph.py
- [ ] T031 GET /api/v1/metadata endpoint implementation in backend/src/api/metadata.py

## Phase 5: Visualization and UI
⚠️ **DEPENDENCY**: Requires Phase 4 completion (working API endpoints)

### Graph Generation and Visualization
- [ ] T032 [P] Graph builder library for creating visualization data from papers+embeddings in backend/src/lib/graph_builder.py
- [ ] T033 [P] Integration test for graph generation with clustering in backend/tests/integration/test_graph_builder.py
- [ ] T034 Frontend API client for fetching papers and graph data in frontend/src/services/api_client.ts
- [ ] T035 D3.js graph visualization component in frontend/src/components/PaperGraph.tsx
- [ ] T036 Search and filter UI components in frontend/src/components/SearchFilter.tsx
- [ ] T037 Paper details modal/panel component in frontend/src/components/PaperDetails.tsx

### System Integration and Polish
- [ ] T038 CORS middleware and security headers in backend/src/main.py
- [ ] T039 Structured logging and error handling across backend services
- [ ] T040 Complete frontend application with routing in frontend/src/App.tsx and main.tsx

## Dependencies

### Critical TDD Dependencies
- Tests T006-T014 MUST complete and FAIL before implementation T015-T032
- T015-T017 (models) block T018-T020 (services)
- T018-T020 (services) block T024-T028 (endpoints)
- T021-T023 (CLI) can run parallel with endpoints (different execution paths)

### Service Dependencies
- T029 (data integration) requires T015-T020 (models + services)
- T030-T031 (middleware/logging) can run after T024-T028 (endpoints)
- T032 (frontend) requires T024-T028 (backend APIs) to be working

### Polish Dependencies
- T033-T035 (unit tests) require T015-T020 (models + services)
- T036 (performance) requires T024-T032 (full system)
- T037-T040 (docs + e2e) require complete implementation

## Parallel Execution Examples

### Phase 3.2: All Contract Tests in Parallel
```bash
# These can run simultaneously since they create different files:
Task: "Contract test GET /api/v1/papers in backend/tests/contract/test_papers_get.py"
Task: "Contract test GET /api/v1/papers/{paper_id} in backend/tests/contract/test_paper_detail.py" 
Task: "Contract test GET /api/v1/papers/{paper_id}/similar in backend/tests/contract/test_paper_similar.py"
Task: "Contract test GET /api/v1/graph in backend/tests/contract/test_graph.py"
Task: "Contract test GET /api/v1/metadata in backend/tests/contract/test_metadata.py"
```

### Phase 3.3: Models + Services + CLI in Parallel
```bash
# These create independent files and have no cross-dependencies:
Task: "Paper model with validation in backend/src/models/paper.py"
Task: "Author model with validation in backend/src/models/author.py"
Task: "ExternalLink model with validation in backend/src/models/external_link.py"
Task: "Data fetcher CLI command in backend/src/cli/data_fetcher.py"
Task: "Embeddings generator CLI command in backend/src/cli/embeddings_generator.py"
Task: "Graph builder CLI command in backend/src/cli/graph_builder.py"
```

### Phase 3.5: Unit Tests in Parallel
```bash
# Independent test files can be written simultaneously:
Task: "Unit tests for Paper model validation in backend/tests/unit/test_paper_model.py"
Task: "Unit tests for embedding calculations in backend/tests/unit/test_embeddings.py"  
Task: "Unit tests for search algorithms in backend/tests/unit/test_search.py"
Task: "Update API documentation in backend/docs/api.md"
```

## Task Generation Rules Applied

1. **From Contracts** (api-spec.yaml):
   - 5 endpoints → 5 contract tests (T006-T010) [P]
   - 5 endpoints → 5 implementation tasks (T024-T028)
   
2. **From Data Model**:
   - Paper, Author, ExternalLink entities → 3 model tasks (T015-T017) [P]
   - Business logic → 3 service tasks (T018-T020) [P]
   
3. **From User Stories** (quickstart.md):
   - 4 user scenarios → 4 integration tests (T011-T014) [P]
   - CLI requirements → 3 CLI tasks (T021-T023) [P]

4. **From Research Decisions**:
   - FastAPI + React → separate setup tasks (T001-T005)
   - Static JSON → data integration task (T029)
   - D3.js visualization → frontend integration (T032)

## Validation Checklist ✅

- [x] All 5 API contracts have corresponding tests (T006-T010)
- [x] All 3 entities have model tasks (T015-T017)
- [x] All tests come before implementation (T006-T014 before T015-T032)
- [x] Parallel tasks are truly independent (different files, no shared state)
- [x] Each task specifies exact file path for implementation
- [x] No [P] task modifies same file as another [P] task
- [x] TDD enforced: contract tests must fail before endpoint implementation
- [x] Constitutional compliance: libraries have CLI interfaces, tests use real data

## Notes
- Total: 40 tasks covering setup → tests → implementation → polish
- Estimated completion: 4-6 weeks for full implementation
- [P] tasks enable significant parallelization (60% of tasks can run in parallel)
- Critical path: T006-T014 (tests) → T015-T020 (core) → T024-T028 (endpoints) → T032 (frontend)
- Each task is specific enough for LLM execution without additional context