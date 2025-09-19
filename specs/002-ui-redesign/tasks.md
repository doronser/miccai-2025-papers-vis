# UI Redesign Tasks

## Phase 1: Core UI Redesign

### Task 1.1: Implement Dark Mode Theme System
- [ ] Create ThemeProvider with React Context
- [ ] Implement useTheme hook for theme access
- [ ] Define CSS custom properties for light/dark themes
- [ ] Add system preference detection (`prefers-color-scheme`)
- [ ] Create ThemeToggle component
- [ ] Add localStorage persistence for user preference
- [ ] Update all components to use theme variables
- [ ] Test color contrast and accessibility (WCAG AA)

### Task 1.2: Refactor GraphVisualization Component
- [ ] Split into ClusterView and SimilarityNetworkView components
- [ ] Implement cluster-first default rendering
- [ ] Add view mode state management
- [ ] Remove edges from default cluster view
- [ ] Add smooth transitions between views

### Task 1.3: Create View Mode Toggle
- [ ] Build ViewModeToggle component
- [ ] Add toggle between "Clusters" and "Similarity Network"
- [ ] Integrate with main App component
- [ ] Add visual indicators for current mode

## Phase 2: Filtering & Selection

### Task 2.1: Subject Area Filter Component
- [ ] Create SubjectAreaFilter component
- [ ] Implement multiselect dropdown
- [ ] Add filter chips display
- [ ] Add "Clear All" functionality
- [ ] Style for dark mode

### Task 2.2: Backend API Updates
- [ ] Add subject_areas parameter to graph endpoint
- [ ] Update GraphParams interface
- [ ] Implement filtering logic in SimilarityService
- [ ] Add validation for subject area filters
- [ ] Update API documentation

### Task 2.3: Paper Selection with Similarity Network
- [ ] Implement click handler for paper selection
- [ ] Create API call for top 20 similar papers
- [ ] Add dynamic edge rendering for selected paper
- [ ] Implement "Back to clusters" functionality
- [ ] Add loading states for similarity network

## Phase 3: Visual Enhancements

### Task 3.1: Node Size and Visual Hierarchy
- [ ] Increase default node radius from 8 to 15-20px
- [ ] Implement size variation based on paper importance
- [ ] Improve node label visibility
- [ ] Add hover effects and animations
- [ ] Optimize rendering performance

### Task 3.2: Comprehensive Legend System
- [ ] Create GraphLegend component
- [ ] Add color explanations for clusters and subject areas
- [ ] Implement interactive legend (click to highlight)
- [ ] Add statistics display (counts per category)
- [ ] Position legend appropriately in dark mode

### Task 3.3: Enhanced Interactions
- [ ] Improve node tooltips with more information
- [ ] Add keyboard navigation support
- [ ] Implement zoom and pan improvements
- [ ] Add animation for view transitions
- [ ] Optimize for large datasets

## Phase 4: Polish & Performance

### Task 4.1: Performance Optimization
- [ ] Implement virtual rendering for large datasets
- [ ] Add debouncing for filter changes
- [ ] Optimize D3.js rendering
- [ ] Add loading states and progress indicators
- [ ] Implement error boundaries

### Task 4.2: Responsive Design
- [ ] Make graph responsive to different screen sizes
- [ ] Optimize mobile experience
- [ ] Test on various devices and browsers
- [ ] Add touch gestures for mobile

### Task 4.3: Testing and Documentation
- [ ] Write unit tests for new components
- [ ] Add integration tests for filtering
- [ ] Update component documentation
- [ ] Create user guide for new features
- [ ] Performance testing with large datasets

## Phase 5: Advanced Features (Optional)

### Task 5.1: Advanced Filtering
- [ ] Add author-based filtering
- [ ] Implement date range filtering
- [ ] Add similarity threshold slider
- [ ] Create saved filter presets

### Task 5.2: Export and Sharing
- [ ] Add graph export functionality
- [ ] Implement shareable URLs with filters
- [ ] Add screenshot export
- [ ] Create embeddable widget

## Dependencies and Prerequisites

### Frontend Dependencies
- React 18+
- D3.js (existing)
- TypeScript (existing)
- CSS-in-JS or styled-components (optional)

### Backend Dependencies
- FastAPI (existing)
- SimilarityService updates
- DataLoader enhancements

### Testing Dependencies
- Jest (existing)
- React Testing Library
- Playwright for E2E tests

## Estimated Timeline
- Phase 1: 1-2 weeks
- Phase 2: 1-2 weeks
- Phase 3: 1-2 weeks
- Phase 4: 1 week
- Phase 5: 1-2 weeks (optional)

**Total: 4-7 weeks** (depending on scope and complexity)
