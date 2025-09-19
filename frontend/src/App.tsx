import { useState, useCallback, useEffect } from 'react';
import TSNEClusterView from './components/TSNEClusterView';
import PaperDetailsPanel from './components/PaperDetailsPanel';
import { ThemeToggle } from './components/ThemeToggle';
import { apiService } from './services/api';
import { Paper } from './types/api';
import './styles/themes.css';

function App() {
  const [searchResults, setSearchResults] = useState<Paper[]>([]);
  const [selectedPaper, setSelectedPaper] = useState<Paper | null>(null);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [sidebarWidth, setSidebarWidth] = useState<number>(400);
  const [isResizing, setIsResizing] = useState(false);

  // No complex initialization needed

  const handleSearch = useCallback(async (query: string) => {
    setIsSearching(true);
    setError(null);
    setSearchQuery(query);

    try {
      const results = await apiService.searchPapers({ q: query, limit: 50 });
      setSearchResults(results);
    } catch (err) {
      setError('Failed to search papers');
      console.error('Error searching papers:', err);
    } finally {
      setIsSearching(false);
    }
  }, []);

  const handleNodeClick = useCallback(async (paperId: string) => {
    try {
      const paper = await apiService.getPaperById(paperId);
      setSelectedPaper(paper);
    } catch (err) {
      console.error('Error loading paper details:', err);
    }
  }, []);


  const handlePaperSelect = useCallback(async (paper: Paper) => {
    setSelectedPaper(paper);
  }, []);

  const handleCloseDetailsPanel = useCallback(() => {
    setSelectedPaper(null);
  }, []);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsResizing(true);
    e.preventDefault();
  }, []);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!isResizing) return;

    const newWidth = window.innerWidth - e.clientX;
    const minWidth = 300;
    const maxWidth = window.innerWidth * 0.6;

    if (newWidth >= minWidth && newWidth <= maxWidth) {
      setSidebarWidth(newWidth);
    }
  }, [isResizing]);

  const handleMouseUp = useCallback(() => {
    setIsResizing(false);
  }, []);

  useEffect(() => {
    if (isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = 'col-resize';
      document.body.style.userSelect = 'none';
    } else {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
  }, [isResizing, handleMouseMove, handleMouseUp]);

  // Handle window resize
  useEffect(() => {
    const handleResize = () => {
      const maxWidth = window.innerWidth * 0.6;
      if (sidebarWidth > maxWidth) {
        setSidebarWidth(maxWidth);
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [sidebarWidth]);

  return (
    <div className="App" style={{
      fontFamily: 'system-ui, -apple-system, sans-serif',
      height: '100vh',
      display: 'flex',
      flexDirection: 'column',
      backgroundColor: 'var(--color-bg-primary)',
      color: 'var(--color-text-primary)'
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
              Interactive t-SNE visualization for exploring conference papers
            </p>
          </div>
          <ThemeToggle />
        </div>
      </header>

      {/* Main Content */}
      <main style={{
        flex: 1,
        display: 'flex',
        padding: '10px',
        gap: '10px',
        height: 'calc(100vh - 120px)',
        overflow: 'hidden',
        width: '100%',
        boxSizing: 'border-box'
      }}>
        {/* Left Column - Search and Visualization */}
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', minHeight: '100%', minWidth: 0 }}>
          {/* Search Bar */}
          <div style={{ marginBottom: '20px' }}>
            <input
              type="text"
              placeholder="Search papers by title, author, or subject area..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSearch(searchQuery)}
              style={{
                width: '100%',
                padding: '12px',
                fontSize: '16px',
                border: '1px solid var(--color-border)',
                borderRadius: '6px',
                backgroundColor: 'var(--color-bg-card)',
                color: 'var(--color-text-primary)'
              }}
            />
            <button
              onClick={() => handleSearch(searchQuery)}
              disabled={isSearching}
              style={{
                marginTop: '10px',
                padding: '10px 20px',
                backgroundColor: 'var(--color-accent)',
                color: 'white',
                border: 'none',
                borderRadius: '6px',
                cursor: 'pointer',
                fontSize: '14px'
              }}
            >
              {isSearching ? 'Searching...' : 'Search'}
            </button>
          </div>

          {/* Search Results */}
          {searchResults.length > 0 && (
            <div style={{ marginBottom: '20px', maxHeight: '200px', overflowY: 'auto' }}>
              <h3 style={{ margin: '0 0 10px 0', fontSize: '16px' }}>Search Results ({searchResults.length})</h3>
              {searchResults.map((paper) => (
                <div
                  key={paper.id}
                  onClick={() => handlePaperSelect(paper)}
                  style={{
                    padding: '10px',
                    marginBottom: '5px',
                    backgroundColor: 'var(--color-bg-card)',
                    border: '1px solid var(--color-border)',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    fontSize: '14px'
                  }}
                >
                  <div style={{ fontWeight: 'bold', marginBottom: '5px' }}>
                    {paper.title}
                  </div>
                  <div style={{ color: 'var(--color-text-secondary)', fontSize: '12px' }}>
                    {paper.authors.map(a => a.name).join(', ')}
                  </div>
                </div>
              ))}
            </div>
          )}

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

          {/* t-SNE Visualization */}
          <div style={{
            flex: 1,
            overflow: 'hidden',
            position: 'relative',
            minHeight: '600px'
          }}>
            <TSNEClusterView
              width={Math.max(400, window.innerWidth - sidebarWidth - 40)}
              height={window.innerHeight - 200}
              onNodeClick={handleNodeClick}
              selectedPaperId={selectedPaper?.id}
              searchQuery={searchQuery}
            />
          </div>
        </div>

        {/* Resize Handle */}
        <div
          onMouseDown={handleMouseDown}
          style={{
            width: '4px',
            backgroundColor: isResizing ? 'var(--color-accent)' : 'var(--color-border)',
            cursor: 'col-resize',
            flexShrink: 0,
            transition: 'background-color var(--transition-fast)'
          }}
        />

        {/* Right Column - Paper Details */}
        <div style={{
          width: `${Math.min(sidebarWidth, window.innerWidth * 0.6)}px`,
          flexShrink: 0,
          display: 'flex',
          flexDirection: 'column',
          maxWidth: '60vw'
        }}>
          {selectedPaper ? (
            <PaperDetailsPanel
              paper={selectedPaper}
              isOpen={true}
              onClose={handleCloseDetailsPanel}
              onPaperSelect={handlePaperSelect}
              isPersistent={true}
            />
          ) : (
            <div style={{
              height: '100%',
              padding: '20px',
              backgroundColor: 'var(--color-bg-card)',
              border: '1px solid var(--color-border)',
              borderRadius: '8px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              textAlign: 'center'
            }}>
              <div>
                <h3 style={{ margin: '0 0 15px 0', color: 'var(--color-text-primary)' }}>
                  Paper Details
                </h3>
                <p style={{ fontSize: '14px', color: 'var(--color-text-secondary)' }}>
                  Search for papers or click on points in the visualization to view details here.
                </p>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
