# Graph Visualization UI Redesign Plan

## Overview
This specification outlines the redesign of the graph visualization component to create a more user-friendly, intuitive, and visually appealing interface for exploring MICCAI 2025 papers.

## Current State Analysis
- Shows all papers with similarity edges by default
- Uses light theme with basic clustering colors
- Small nodes (radius 8) with minimal visual hierarchy
- No filtering capabilities beyond search
- Limited legend information

## Key Improvements

### 1. Dark Mode Theme System 🌙
- **Theme Context**: React Context for theme management
- **CSS Custom Properties**: Dynamic theme switching with CSS variables
- **System Preference**: Respect user's OS dark mode preference (`prefers-color-scheme`)
- **Manual Toggle**: Theme switcher component in UI
- **Persistent Storage**: Save user preference in localStorage
- **High Contrast**: Bright, accessible colors for dark backgrounds

### 2. Cluster-First Default View 🎯
- **Default state**: Show papers grouped by clusters with NO edges
- **Visual clustering**: Use spatial positioning to group similar papers
- **Cluster labels**: Show cluster names/topics
- **Toggle option**: Switch between "Cluster View" and "Similarity Network"

### 3. Subject Area Filtering 🔍
- **Multiselect dropdown**: Filter by subject areas
- **Real-time filtering**: Update graph immediately when filters change
- **Filter chips**: Show active filters with remove buttons
- **"All Areas" option**: Clear all filters

### 4. Smart Paper Selection 🎯
- **Click behavior**: When selecting a paper, show only top 20 most similar papers
- **Dynamic edges**: Add similarity edges only for the selected paper's network
- **Focused view**: Dim or hide non-relevant papers
- **"Back to clusters" button**: Return to cluster view

### 5. Enhanced Visual Design ✨
- **Larger nodes**: Increase default radius from 8 to 15-20px
- **Size variation**: Use node size to indicate importance/popularity
- **Better typography**: Larger, more readable labels
- **Smooth animations**: Transitions between different views

### 6. Comprehensive Legend 📊
- **Color explanation**:
  - KMeans clusters (if available)
  - Subject area colors
  - Node size meaning
- **Interactive legend**: Click to highlight specific clusters/areas
- **Statistics**: Show counts for each category

## Technical Implementation

### Component Structure
```
components/
├── providers/
│   └── ThemeProvider.tsx (new)
├── hooks/
│   └── useTheme.ts (new)
├── GraphVisualization.tsx (refactored)
├── ClusterView.tsx (new)
├── SimilarityNetworkView.tsx (new)
├── SubjectAreaFilter.tsx (new)
├── GraphLegend.tsx (new)
├── ViewModeToggle.tsx (new)
├── ThemeToggle.tsx (new)
└── NodeTooltip.tsx (new)
```

### API Enhancements
- `GET /papers/graph/data?subject_areas[]=...` - Filter by subject areas
- `GET /papers/{id}/similar?limit=20` - Get top 20 similar papers
- `GET /papers/clusters/` - Get cluster information

## Success Metrics
- Improved user engagement with graph visualization
- Faster discovery of relevant papers
- Better visual hierarchy and readability
- Reduced cognitive load through clustering
