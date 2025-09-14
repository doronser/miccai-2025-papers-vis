# Tasks: MICCAI 2025 Papers Visualization Webapp

**Input**: Design documents from `/specs/001-plan-a-webapp/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Tech stack: Python 3.11+/FastAPI backend, React 18+/TypeScript frontend
   → Libraries: sentence-transformers, D3.js, Vite
   → Structure: Web app (backend/ + frontend/)
2. Load design documents:
   → data-model.md: Paper, Author, ExternalLink entities
   → contracts/api-spec.yaml: 5 endpoints with OpenAPI spec
   → research.md: Technology decisions validated
3. Generate tasks by category:
   → Setup: project structure, dependencies, linting
   → Tests: 5 contract tests, 4 integration tests
   → Core: 3 models, 3 services, 3 CLI commands, 5 endpoints
   → Integration: data pipeline, graph generation, search
   → Polish: unit tests, performance validation, docs
4. Apply task rules:
   → Different files = [P] parallel
   → TDD enforced: tests before implementation
5. Number tasks T001-T032
6. Validate completeness: All contracts tested, entities modeled
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- File paths use web app structure: `backend/src/`, `frontend/src/`

## Phase 3.1: Setup
- [ ] T001 Create backend/ and frontend/ directory structure per implementation plan
- [ ] T002 Initialize Python backend with FastAPI, sentence-transformers, pytest dependencies in backend/requirements.txt
- [ ] T003 Initialize React frontend with TypeScript, D3.js, Vite dependencies in frontend/package.json
- [ ] T004 [P] Configure Python linting (ruff, black) in backend/.pre-commit-config.yaml
- [ ] T005 [P] Configure TypeScript/ESLint in frontend/.eslintrc.json and frontend/tsconfig.json

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests (API Endpoints)
- [ ] T006 [P] Contract test GET /api/v1/papers in backend/tests/contract/test_papers_get.py
- [ ] T007 [P] Contract test GET /api/v1/papers/{paper_id} in backend/tests/contract/test_paper_detail.py
- [ ] T008 [P] Contract test GET /api/v1/papers/{paper_id}/similar in backend/tests/contract/test_paper_similar.py
- [ ] T009 [P] Contract test GET /api/v1/graph in backend/tests/contract/test_graph.py
- [ ] T010 [P] Contract test GET /api/v1/metadata in backend/tests/contract/test_metadata.py

### Integration Tests (User Stories)
- [ ] T011 [P] Integration test "explore papers via graph" user journey in backend/tests/integration/test_graph_exploration.py
- [ ] T012 [P] Integration test "search and filter papers" user flow in backend/tests/integration/test_search_filter.py
- [ ] T013 [P] Integration test "find similar papers" functionality in backend/tests/integration/test_similarity.py
- [ ] T014 [P] Integration test "data pipeline from MICCAI to embeddings" in backend/tests/integration/test_data_pipeline.py

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Models (Data Entities)
- [ ] T015 [P] Paper model with validation in backend/src/models/paper.py
- [ ] T016 [P] Author model with validation in backend/src/models/author.py
- [ ] T017 [P] ExternalLink model with validation in backend/src/models/external_link.py

### Services (Business Logic)
- [ ] T018 [P] PaperService for CRUD operations in backend/src/services/paper_service.py
- [ ] T019 [P] EmbeddingsService for similarity calculations in backend/src/services/embeddings_service.py
- [ ] T020 [P] SearchService for filtering and queries in backend/src/services/search_service.py

### CLI Commands (Library Interface)
- [ ] T021 [P] Data fetcher CLI command in backend/src/cli/data_fetcher.py
- [ ] T022 [P] Embeddings generator CLI command in backend/src/cli/embeddings_generator.py
- [ ] T023 [P] Graph builder CLI command in backend/src/cli/graph_builder.py

### API Endpoints
- [ ] T024 GET /api/v1/papers endpoint implementation in backend/src/api/papers.py
- [ ] T025 GET /api/v1/papers/{paper_id} endpoint implementation in backend/src/api/papers.py
- [ ] T026 GET /api/v1/papers/{paper_id}/similar endpoint implementation in backend/src/api/papers.py
- [ ] T027 GET /api/v1/graph endpoint implementation in backend/src/api/graph.py
- [ ] T028 GET /api/v1/metadata endpoint implementation in backend/src/api/metadata.py

## Phase 3.4: Integration
- [ ] T029 Connect services to static JSON data storage in backend/src/data/
- [ ] T030 CORS middleware and security headers in backend/src/main.py
- [ ] T031 Structured logging and error handling across backend services
- [ ] T032 Frontend API client and graph visualization in frontend/src/

## Phase 3.5: Polish
- [ ] T033 [P] Unit tests for Paper model validation in backend/tests/unit/test_paper_model.py
- [ ] T034 [P] Unit tests for embedding calculations in backend/tests/unit/test_embeddings.py
- [ ] T035 [P] Unit tests for search algorithms in backend/tests/unit/test_search.py
- [ ] T036 Performance tests ensuring <2s load time, <500ms API responses in backend/tests/performance/
- [ ] T037 [P] Update API documentation in backend/docs/api.md
- [ ] T038 Execute quickstart.md validation scenarios and fix any issues
- [ ] T039 Frontend unit tests for graph components in frontend/tests/unit/
- [ ] T040 End-to-end tests for complete user workflows in tests/e2e/

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