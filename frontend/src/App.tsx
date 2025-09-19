import React, { useState, useEffect, useCallback } from 'react';
import GraphVisualization from './components/GraphVisualization';
import SearchPanel from './components/SearchPanel';
import PaperDetailsPanel from './components/PaperDetailsPanel';
import { ThemeToggle } from './components/ThemeToggle';
import SubjectAreaFilter from './components/SubjectAreaFilter';
import ApiService from './services/api';
import { useFavorites } from './hooks/useFavorites';
import { Paper, GraphData, GraphNode, GraphEdge, DatasetStats } from './types/api';
import './styles/themes.css';

function App() {
  const [graphData, setGraphData] = useState<GraphData | null>(null);
  const [searchResults, setSearchResults] = useState<Paper[]>([]);
  const [selectedPaper, setSelectedPaper] = useState<Paper | null>(null);
  const [isDetailsPanelOpen, setIsDetailsPanelOpen] = useState(false);
  const [isLoadingGraph, setIsLoadingGraph] = useState(false);
  const [isSearching, setIsSearching] = useState(false);
  const [datasetStats, setDatasetStats] = useState<DatasetStats | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedSubjectAreas, setSelectedSubjectAreas] = useState<string[]>([]);
  const [selectedPaperId, setSelectedPaperId] = useState<string | null>(null);
  const [similarityViewData, setSimilarityViewData] = useState<GraphData | null>(null);

  const { favorites, toggleFavorite } = useFavorites();

  // Load graph data on mount
  useEffect(() => {
    loadGraphData();
    loadDatasetStats();
  }, []);

  const loadGraphData = useCallback(async (subjectAreas?: string[], mode: 'clusters' | 'network' = 'clusters') => {
    setIsLoadingGraph(true);
    setError(null);

    try {
      if (mode === 'clusters') {
        // Load similarity-based clusters data - shows all papers
        const data = await ApiService.getClustersData(subjectAreas, 500); // Sample size for performance
        setGraphData(data);
      } else {
        // Load network data for similarity mode
        const data = await ApiService.getGraphData({
          similarity_threshold: 0.75,
          max_edges: 500,
          sample_size: 200,
          subject_areas: subjectAreas
        });
        setGraphData(data);
      }
    } catch (err) {
      setError('Failed to load graph data');
      console.error('Error loading graph data:', err);
    } finally {
      setIsLoadingGraph(false);
    }
  }, []);

  const loadDatasetStats = useCallback(async () => {
    try {
      const stats = await ApiService.getDatasetStats();
      setDatasetStats(stats);
    } catch (err) {
      console.error('Error loading dataset stats:', err);
    }
  }, []);

  const handleSearch = useCallback(async (query: string) => {
    setIsSearching(true);
    setError(null);

    try {
      const results = await ApiService.searchPapers({ q: query, limit: 50 });
      setSearchResults(results);
    } catch (err) {
      setError('Failed to search papers');
      console.error('Error searching papers:', err);
    } finally {
      setIsSearching(false);
    }
  }, []);

  const loadSimilarityViewData = useCallback(async (paperId: string) => {
    setIsLoadingGraph(true);
    setError(null);

    try {
      // Use the new network API to get similarity data
      const networkData = await ApiService.getNetworkData(paperId, 20);
      setSimilarityViewData(networkData);
    } catch (err) {
      setError('Failed to load similarity data');
      console.error('Error loading similarity data:', err);
    } finally {
      setIsLoadingGraph(false);
    }
  }, []);

  const handleNodeClick = useCallback(async (node: GraphNode) => {
    try {
      // Load paper details for the details panel
      const paper = await ApiService.getPaperById(node.id);
      setSelectedPaper(paper);
      setIsDetailsPanelOpen(true);

      // Load similarity view data
      setSelectedPaperId(node.id);
      await loadSimilarityViewData(node.id);
    } catch (err) {
      console.error('Error loading paper details:', err);
    }
  }, [loadSimilarityViewData]);

  const handlePaperSelect = useCallback(async (paper: Paper) => {
    setSelectedPaper(paper);
    setIsDetailsPanelOpen(true);

    // Also load similarity view data to show the paper on the graph
    setSelectedPaperId(paper.id);
    await loadSimilarityViewData(paper.id);
  }, [loadSimilarityViewData]);

  const handleCloseDetailsPanel = useCallback(() => {
    setIsDetailsPanelOpen(false);
    setSelectedPaper(null);
    // Also clear the similarity view when closing details panel
    setSelectedPaperId(null);
    setSimilarityViewData(null);
  }, []);

  const handleSubjectAreaChange = useCallback((areas: string[]) => {
    setSelectedSubjectAreas(areas);
    loadGraphData(areas.length > 0 ? areas : undefined);
  }, [loadGraphData]);

  const handleViewModeChange = useCallback((mode: 'clusters' | 'network') => {
    loadGraphData(selectedSubjectAreas.length > 0 ? selectedSubjectAreas : undefined, mode);
  }, [loadGraphData, selectedSubjectAreas]);

  const handleBackToClusters = useCallback(() => {
    setSelectedPaperId(null);
    setSimilarityViewData(null);
    // Reload the original graph data
    loadGraphData(selectedSubjectAreas.length > 0 ? selectedSubjectAreas : undefined);
  }, [loadGraphData, selectedSubjectAreas]);

  // Get available subject areas from dataset stats
  const availableSubjectAreas = React.useMemo(() => {
    if (!datasetStats?.subject_areas) return [];
    return Object.keys(datasetStats.subject_areas).sort();
  }, [datasetStats]);

  return (
    <div className="App" style={{
      fontFamily: 'system-ui, -apple-system, sans-serif',
      height: '100vh',
      display: 'flex',
      flexDirection: 'column',
      backgroundColor: 'var(--color-bg-primary)',
      color: 'var(--color-text-primary)',
      transition: 'background-color var(--transition-normal), color var(--transition-normal)'
    }}>
      {/* Header */}
      <header style={{
        padding: '20px',
        backgroundColor: 'var(--color-bg-card)',
        borderBottom: '2px solid var(--color-border)',
        boxShadow: 'var(--shadow-md)'
      }}>
        <div style={{ maxWidth: '1200px', margin: '0 auto', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <h1 style={{
              margin: '0 0 8px 0',
              fontSize: '28px',
              color: 'var(--color-text-primary)'
            }}>
              MICCAI 2025 Papers Visualization
            </h1>
            <p style={{
              margin: '0',
              fontSize: '16px',
              color: 'var(--color-text-secondary)'
            }}>
              Interactive graph visualization for exploring {datasetStats?.total_papers || ''} conference papers
              {favorites.size > 0 && (
                <span style={{ marginLeft: '20px', color: 'var(--color-error)' }}>
                  ♥ {favorites.size} favorites
                </span>
              )}
            </p>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            <SubjectAreaFilter
              availableAreas={availableSubjectAreas}
              selectedAreas={selectedSubjectAreas}
              onSelectionChange={handleSubjectAreaChange}
              isLoading={isLoadingGraph}
            />
            <ThemeToggle />
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main style={{
        flex: 1,
        display: 'flex',
        padding: '10px',
        maxWidth: '100%',
        margin: '0 auto',
        width: '100%',
        position: 'relative',
        gap: '10px',
        height: 'calc(100vh - 120px)' // Use most of the viewport height
      }}>
        {/* Left Column - Graph (Main Element) */}
        <div style={{ flex: 3, display: 'flex', flexDirection: 'column', minHeight: '100%' }}>
          {/* Search Panel */}
          <SearchPanel
            onSearch={handleSearch}
            searchResults={searchResults}
            isSearching={isSearching}
            onPaperSelect={handlePaperSelect}
            selectedPaperId={selectedPaperId}
          />

          {/* Error Display */}
          {error && (
            <div style={{
              padding: '15px',
              backgroundColor: 'var(--color-error)',
              color: 'white',
              border: '1px solid var(--color-error)',
              borderRadius: '6px',
              marginBottom: '20px'
            }}>
              {error}
            </div>
          )}

          {/* Graph Container */}
          <div className="graph-container" style={{
            flex: 1,
            overflow: 'hidden',
            position: 'relative',
            minHeight: '600px'
          }}>
            {isLoadingGraph ? (
              <div style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                height: '100%',
                fontSize: '18px',
                color: 'var(--color-text-secondary)'
              }}>
                <div>
                  <div style={{ marginBottom: '10px' }}>Loading graph visualization...</div>
                  <div style={{ fontSize: '14px' }}>
                    Processing {datasetStats?.total_papers || ''} papers and their relationships
                  </div>
                </div>
              </div>
            ) : (similarityViewData || graphData) ? (
              <div style={{ position: 'relative' }}>
                {/* Back to Clusters Button */}
                {selectedPaperId && (
                  <div style={{
                    position: 'absolute',
                    top: 10,
                    left: 10,
                    zIndex: 10
                  }}>
                    <button
                      onClick={handleBackToClusters}
                      className="btn btn-primary"
                      style={{
                        padding: '8px 16px',
                        fontSize: '14px',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px'
                      }}
                    >
                      ← Back to Clusters
                    </button>
                  </div>
                )}

                <GraphVisualization
                  data={similarityViewData || graphData!}
                  onNodeClick={handleNodeClick}
                  width={window.innerWidth - 400} // Much larger width
                  height={window.innerHeight - 200} // Much larger height
                  initialViewMode={selectedPaperId ? 'similarity' : 'clusters'}
                  selectedPaperId={selectedPaperId}
                  onViewModeChange={handleViewModeChange}
                />
              </div>
            ) : (
              <div style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                height: '100%',
                fontSize: '16px',
                color: 'var(--color-text-secondary)',
                flexDirection: 'column'
              }}>
                <div style={{ marginBottom: '15px' }}>
                  Failed to load graph data
                </div>
                <button
                  onClick={() => loadGraphData()}
                  className="btn btn-primary"
                >
                  Retry
                </button>
              </div>
            )}
          </div>

          {/* Dataset Stats */}
          {datasetStats && (
            <div className="card" style={{
              marginTop: '20px',
              padding: '15px'
            }}>
              <div style={{
                display: 'grid',
                gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
                gap: '20px',
                fontSize: '14px'
              }}>
                <div>
                  <strong>Total Papers:</strong> {datasetStats.total_papers.toLocaleString()}
                </div>
                <div>
                  <strong>Total Authors:</strong> {datasetStats.total_authors.toLocaleString()}
                </div>
                <div>
                  <strong>Subject Areas:</strong> {Object.keys(datasetStats.subject_areas).length}
                </div>
                <div>
                  <strong>Top Area:</strong> {
                    Object.entries(datasetStats.subject_areas)[0]?.[0] || 'N/A'
                  }
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Right Column - Persistent Sidebar */}
        <div style={{
          width: '400px',
          flexShrink: 0,
          display: 'flex',
          flexDirection: 'column'
        }}>
          {selectedPaper ? (
            <PaperDetailsPanel
              paper={selectedPaper}
              isOpen={true}
              onClose={handleCloseDetailsPanel}
              onPaperSelect={handlePaperSelect}
              favorites={favorites}
              onToggleFavorite={toggleFavorite}
              isPersistent={true}
            />
          ) : (
            <div className="card" style={{
              height: '100%',
              padding: '20px',
              backgroundColor: 'var(--color-bg-card)',
              border: '1px solid var(--color-border)',
              borderRadius: '8px'
            }}>
              <h3 style={{ margin: '0 0 15px 0', color: 'var(--color-text-primary)' }}>
                Search Results
              </h3>
              {searchResults.length > 0 ? (
                <div style={{ fontSize: '14px', color: 'var(--color-text-secondary)' }}>
                  {searchResults.length} papers found. Select a paper from the search results above to view details here.
                </div>
              ) : (
                <div style={{ fontSize: '14px', color: 'var(--color-text-secondary)' }}>
                  Use the search bar to find papers, then select one to view its details here.
                </div>
              )}
            </div>
          )}
        </div>
      </main>

    </div>
  );
}

export default App;
