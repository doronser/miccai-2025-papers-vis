# Graph Visualization Rebuild Plan

Based on the comprehensive embeddings analysis notebook, this document outlines the complete rebuild of the React webapp's graph visualization system.

## Current State Analysis

### Current Implementation Issues:
1. **ClusterView**: Uses force simulation instead of t-SNE coordinates
2. **SimilarityNetworkView**: Shows all papers with edges, not focused similarity view
3. **Paper Selection**: Automatically switches to similarity view, losing cluster context
4. **No Highlighting**: No way to highlight specific papers or subject areas
5. **Poor Default View**: Points not optimally distributed for full dataset visibility

## Proposed Changes

### 1. Cluster Visualization (Main View)

#### Core Changes:
- **Use t-SNE coordinates** from embeddings analysis instead of force simulation
- **Uniform coloring** with subject area highlighting capability
- **Auto-fit view** to show all points optimally by default
- **Paper highlighting** system for selected papers
- **Subject area filtering** with visual highlighting

#### Implementation Details:

**New Component: `TSNEClusterView.tsx`**
```typescript
interface TSNEClusterViewProps {
  data: GraphData;
  width: number;
  height: number;
  onNodeClick?: (node: GraphNode) => void;
  onNodeHover?: (node: GraphNode | null) => void;
  selectedPaperId?: string | null;
  highlightedSubjectAreas?: string[];
  onSubjectAreaHighlight?: (areas: string[]) => void;
}
```

**Key Features:**
- Load t-SNE coordinates from backend API
- Auto-scale and center to fit all points in view
- Uniform node coloring with highlight states
- Subject area filtering with visual feedback
- Smooth transitions between states

**Backend API Changes:**
```typescript
// New endpoint: GET /api/papers/tsne-coordinates
interface TSNECoordinates {
  paper_id: string;
  tsne_x: number;
  tsne_y: number;
  subject_areas: string[];
  cluster_id?: number;
}
```

### 2. Similarity View (Paper-Focused)

#### Core Changes:
- **Top 20 similar papers** visualization only
- **Distance-annotated edges** showing cosine similarity
- **Target paper highlighting** with star marker
- **Network layout** optimized for similarity relationships

#### Implementation Details:

**Enhanced Component: `SimilarityNetworkView.tsx`**
```typescript
interface SimilarityNetworkViewProps {
  targetPaperId: string;
  similarPapers: PaperSimilarity[];
  width: number;
  height: number;
  onNodeClick?: (node: GraphNode) => void;
  onNodeHover?: (node: GraphNode | null) => void;
}
```

**Key Features:**
- Load top 20 similar papers via API
- Apply t-SNE to similarity subset
- Draw edges with distance annotations
- Highlight target paper prominently
- Show similarity scores in tooltips

**Backend API Changes:**
```typescript
// Enhanced endpoint: GET /api/papers/{id}/similarity-network
interface SimilarityNetworkData {
  target_paper: Paper;
  similar_papers: PaperSimilarity[];
  tsne_coordinates: {
    paper_id: string;
    tsne_x: number;
    tsne_y: number;
    similarity_score: number;
  }[];
  distance_matrix: number[][];
}
```

### 3. Paper Selection & Details

#### Core Changes:
- **Separate paper details** from graph view changes
- **Dedicated "Show Similar Papers" button** in details panel
- **Persistent cluster view** when viewing paper details
- **Highlight selected paper** in cluster view without switching

#### Implementation Details:

**Updated `PaperDetailsPanel.tsx`:**
```typescript
interface PaperDetailsPanelProps {
  paper?: Paper | null;
  isOpen: boolean;
  onClose: () => void;
  onShowSimilarPapers?: (paperId: string) => void; // New callback
  onHighlightPaper?: (paperId: string) => void; // New callback
  // ... existing props
}
```

**New Features:**
- "Show Similar Papers" button in details panel
- "Highlight in Graph" button for cluster view
- Paper details persist when switching views
- Clear visual feedback for selected state

### 4. Enhanced User Experience

#### New Controls:
- **Subject Area Filter**: Multi-select dropdown with highlighting
- **Paper Search**: Search and highlight specific papers
- **View Controls**: Toggle between cluster and similarity views
- **Zoom Controls**: Reset view, fit to data, zoom to selection

#### Implementation Details:

**New Component: `GraphControls.tsx`**
```typescript
interface GraphControlsProps {
  availableSubjectAreas: string[];
  selectedSubjectAreas: string[];
  onSubjectAreaChange: (areas: string[]) => void;
  onSearchPaper: (query: string) => void;
  onResetView: () => void;
  onFitToData: () => void;
}
```

## Technical Implementation Plan

### Phase 1: Backend API Updates

1. **Add t-SNE coordinates endpoint**
   ```python
   @router.get("/papers/tsne-coordinates")
   async def get_tsne_coordinates():
       # Load precomputed t-SNE coordinates from notebook analysis
       # Return coordinates for all papers
   ```

2. **Enhance similarity network endpoint**
   ```python
   @router.get("/papers/{paper_id}/similarity-network")
   async def get_similarity_network(paper_id: str, top_k: int = 20):
       # Use cosine similarity matrix from notebook
       # Apply t-SNE to subset
       # Return network data with coordinates
   ```

3. **Add paper highlighting endpoint**
   ```python
   @router.get("/papers/{paper_id}/highlight")
   async def highlight_paper(paper_id: str):
       # Return paper data for highlighting in cluster view
   ```

### Phase 2: Frontend Component Updates

1. **Create `TSNEClusterView.tsx`**
   - Replace force simulation with t-SNE coordinates
   - Implement auto-fitting and scaling
   - Add highlighting system
   - Uniform coloring with subject area filtering

2. **Update `SimilarityNetworkView.tsx`**
   - Focus on top 20 similar papers only
   - Add distance-annotated edges
   - Implement network layout optimization
   - Target paper highlighting

3. **Enhance `PaperDetailsPanel.tsx`**
   - Add "Show Similar Papers" button
   - Add "Highlight in Graph" button
   - Separate details from graph view changes

4. **Create `GraphControls.tsx`**
   - Subject area filtering
   - Paper search functionality
   - View control buttons
   - Zoom and fit controls

### Phase 3: Integration & Testing

1. **Update `App.tsx`**
   - Integrate new components
   - Update state management
   - Handle view transitions
   - Maintain paper selection state

2. **Update `GraphVisualization.tsx`**
   - Coordinate between cluster and similarity views
   - Handle highlighting and filtering
   - Manage view state transitions

3. **Testing & Optimization**
   - Performance testing with large datasets
   - User experience testing
   - Responsive design validation
   - Cross-browser compatibility

## Data Flow Architecture

```
User Action → Component → API Call → Backend Processing → Response → Visualization Update

1. Load Cluster View:
   User opens app → TSNEClusterView → GET /papers/tsne-coordinates → Load coordinates → Render points

2. Select Paper:
   User clicks paper → PaperDetailsPanel → GET /papers/{id} → Show details + highlight in cluster

3. Show Similar Papers:
   User clicks "Show Similar" → SimilarityNetworkView → GET /papers/{id}/similarity-network → Render network

4. Filter by Subject Area:
   User selects areas → TSNEClusterView → Filter coordinates → Update visualization
```

## Benefits of New Architecture

1. **Better Performance**: Precomputed t-SNE coordinates eliminate client-side computation
2. **Improved UX**: Clear separation between cluster overview and similarity analysis
3. **Enhanced Interactivity**: Multiple ways to explore and highlight papers
4. **Scalability**: Efficient handling of large datasets with precomputed embeddings
5. **Consistency**: Aligned with notebook analysis methodology

## Migration Strategy

1. **Parallel Development**: Build new components alongside existing ones
2. **Feature Flags**: Use feature flags to toggle between old and new implementations
3. **Gradual Rollout**: Deploy new features incrementally
4. **Fallback Support**: Maintain old implementation as fallback
5. **User Feedback**: Collect feedback during development and adjust accordingly

## Success Metrics

1. **Performance**: Page load time < 2 seconds for cluster view
2. **Usability**: Users can find and explore papers within 3 clicks
3. **Visual Quality**: All points visible in default view
4. **Responsiveness**: Smooth interactions and transitions
5. **Accessibility**: Keyboard navigation and screen reader support

This plan provides a comprehensive roadmap for rebuilding the graph visualization system based on the insights from the embeddings analysis notebook, ensuring a more intuitive and powerful user experience.
