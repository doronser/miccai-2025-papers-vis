# Quickstart Guide: MICCAI 2025 Papers Visualization

## Prerequisites
- Python 3.11+ installed
- Node.js 18+ installed
- Git for version control

## Quick Setup (Development)

### 1. Clone and Setup
```bash
git clone <repository-url>
cd miccai-2025-papers-vis

# Backend setup
cd backend
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

# Frontend setup (in new terminal)
cd frontend
npm install
```

### 2. Data Preparation
```bash
# From backend directory
python -m cli.data_fetcher --source miccai --output data/papers.json
python -m cli.embeddings_generator --input data/papers.json --output data/papers_with_embeddings.json
```

### 3. Start Development Servers
```bash
# Backend (Terminal 1)
cd backend
python -m uvicorn main:app --reload --host 0.0.0.0 --port 8000

# Frontend (Terminal 2) 
cd frontend
npm run dev
```

### 4. Verify Setup
- Backend API docs: http://localhost:8000/docs
- Frontend app: http://localhost:3000
- Health check: http://localhost:8000/api/v1/metadata

## User Journey Validation

### Story 1: Explore Papers via Graph
1. **Open** http://localhost:3000
2. **Verify** graph visualization loads with ~1000 paper nodes
3. **Click** any paper node
4. **Confirm** paper details panel shows title, abstract, authors
5. **Check** external links are clickable and valid

**Expected Result**: Visual graph with interactive nodes, readable paper information

### Story 2: Search and Filter Papers
1. **Type** "deep learning" in search box
2. **Verify** graph updates to highlight matching papers
3. **Select** "Computer Vision" from subject area filter
4. **Confirm** results combine search + filter criteria
5. **Clear** filters and verify full graph returns

**Expected Result**: Responsive search with visual feedback, combined filtering works

### Story 3: Find Similar Papers
1. **Click** on any paper node
2. **Click** "Find Similar" button in details panel
3. **Verify** related papers are highlighted in graph
4. **Check** similarity scores are displayed (0.0-1.0)
5. **Click** a similar paper to navigate

**Expected Result**: Related papers visually highlighted, similarity scores accurate

### Story 4: Save Favorites (Static Version)
1. **Click** heart icon on interesting paper
2. **Verify** paper is added to favorites sidebar
3. **Navigate** to different papers
4. **Confirm** favorites list persists in session
5. **Click** favorite item to return to paper

**Expected Result**: Local favorites storage works, navigation between favorites smooth

## API Validation Tests

### Core Endpoints
```bash
# Get all papers
curl "http://localhost:8000/api/v1/papers?limit=10"

# Search papers
curl "http://localhost:8000/api/v1/papers?search=deep%20learning"

# Get specific paper
curl "http://localhost:8000/api/v1/papers/paper-001"

# Get similar papers
curl "http://localhost:8000/api/v1/papers/paper-001/similar?limit=5"

# Get graph data
curl "http://localhost:8000/api/v1/graph?max_nodes=100"

# Get metadata
curl "http://localhost:8000/api/v1/metadata"
```

### Expected Response Validation
- All endpoints return JSON
- Status codes: 200 for success, 404 for not found, 400 for bad request
- Response schemas match OpenAPI specification
- Pagination works for large datasets
- Error messages are descriptive

## Performance Validation

### Load Time Tests
- **Initial page load**: < 3 seconds
- **Graph rendering**: < 2 seconds for 1000 nodes
- **Search results**: < 500ms response time
- **Paper details**: < 200ms to display

### Memory Usage
- **Frontend**: < 200MB in Chrome dev tools
- **Backend**: < 500MB Python process
- **Data files**: < 50MB JSON storage

## Common Issues & Solutions

### Backend Won't Start
```bash
# Check Python version
python --version  # Should be 3.11+

# Reinstall dependencies
pip install -r requirements.txt --force-reinstall

# Check port availability
lsof -i :8000
```

### Frontend Build Errors
```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Check Node version
node --version  # Should be 18+
```

### Graph Not Rendering
1. Check browser console for JavaScript errors
2. Verify API endpoints return data: `curl localhost:8000/api/v1/papers`
3. Check CORS settings in backend configuration
4. Ensure D3.js library loaded properly

### Missing Paper Data
```bash
# Re-run data fetching pipeline
cd backend
python -m cli.data_fetcher --source miccai --force-refresh
python -m cli.embeddings_generator --input data/papers.json --output data/papers_with_embeddings.json --force-rebuild
```

## Deployment Checklist

### Production Readiness
- [ ] Environment variables configured
- [ ] Database connections tested (if using DB)
- [ ] Static assets optimized and compressed
- [ ] API rate limiting configured
- [ ] Error logging and monitoring setup
- [ ] Health check endpoints working
- [ ] CORS policies configured for production domains

### Static Site Deployment
```bash
# Build frontend for production
cd frontend
npm run build

# Test production build locally
npm run preview

# Deploy to Vercel/Netlify
# Follow platform-specific deployment guides
```

This quickstart guide ensures developers can rapidly set up the development environment and validate all core functionality works as specified in the feature requirements.