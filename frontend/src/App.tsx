import { useState, useCallback, useEffect, useMemo } from 'react';
import TSNEClusterView from './components/TSNEClusterView';
import PaperDetailsPanel from './components/PaperDetailsPanel';
import MobileWarning from './components/MobileWarning';
import { ThemeToggle } from './components/ThemeToggle';
import { HamburgerMenu } from './components/HamburgerMenu';
import { MobileSidebar } from './components/MobileSidebar';
import { FloatingActionButton } from './components/FloatingActionButton';
import { InfoPopup } from './components/InfoPopup';
import SubjectAreaFilter from './components/SubjectAreaFilter';
import SimilarPapersSection from './components/SimilarPapersSection';
import { useMobile } from './hooks/useMobile';
import { apiService } from './services/api';
import { Paper } from './types/api';
import './styles/themes.css';

function App() {
  const [searchResults, setSearchResults] = useState<Paper[]>([]);
  const [selectedPaper, setSelectedPaper] = useState<Paper | null>(null);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [selectedSubjectAreas, setSelectedSubjectAreas] = useState<string[]>([]);
  const [sidebarWidth, setSidebarWidth] = useState<number>(400);
  const [isResizing, setIsResizing] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isSearchFocused, setIsSearchFocused] = useState(false);
  const [isInfoPopupOpen, setIsInfoPopupOpen] = useState(false);
  const [isPaperDetailsExpanded, setIsPaperDetailsExpanded] = useState(true);
  const [isSimilarPapersExpanded, setIsSimilarPapersExpanded] = useState(true);

  // Available subject areas from the dataset
  const availableSubjectAreas = [
    'Machine Learning -> Deep Learning',
    'Applications -> Computer Aided Diagnosis',
    'Modalities -> CT / X-Ray',
    'Modalities -> MRI',
    'Body -> Brain',
    'Applications -> Image Segmentation',
    'Body -> other',
    'Applications -> Image Synthesis / Augmentation / Super-Resolution',
    'Body -> Abdomen',
    'Machine Learning -> Semi- / Weakly- / Self-supervised Learning',
    'Body -> Lung',
    'Machine Learning -> Foundation Models',
    'Modalities -> Other',
    'Applications -> Other',
    'Modalities -> Photograph / Video',
    'Modalities -> Microscopy',
    'Body -> Breast',
    'Body -> Cardiac',
    'Machine Learning -> Interpretability / Explainability',
    'Applications -> Computational (Integrative) Pathology',
    'Modalities -> Endoscopy',
    'Body -> Eye',
    'Machine Learning -> Domain Adaptation / Harmonization',
    'Modalities -> Ultrasound',
    'Applications -> Anomaly Detection',
    'Surgery -> Data Science',
    'Applications -> Image-Guided Interventions',
    'Applications -> Brain Network Analysis',
    'Body -> Skin',
    'Machine Learning -> Uncertainty',
    'Modalities -> MRI - Functional MRI',
    'Applications -> Image Registration',
    'Applications -> Integration of Imaging with Non-Imaging Biomarkers',
    'Machine Learning -> Other',
    'Surgery -> Scene Understanding',
    'Special Topic -> MIC and CAI Solutions for Limited-Resource Environments',
    'Applications -> Visualization in Biomedical Imaging',
    'Surgery -> Navigation',
    'Modalities -> Nuclear Imaging',
    'Machine Learning -> Validation',
    'Surgery -> Planning and Simulation',
    'Modalities -> EEG / ECG',
    'Surgery -> Mixed / Augmented / Virtual Reality',
    'Body -> Vasculature',
    'Body -> Fetal / Pediatric Imaging',
    'Applications -> Computational Anatomy and Physiology',
    'Special Topic -> Low-Cost and Point-of-Care Imaging Solutions',
    'Modalities -> MRI - Diffusion Imaging',
    'Body -> Spine',
    'Surgery -> Other',
    'Modalities -> Robotics',
    'Machine Learning -> Algorithmic Fairness',
    'Body -> Urology',
    'Surgery -> Skill and Work Flow Analysis',
    'Modalities -> Spectroscopy',
    'Body -> Gynecology',
    'Special Topic -> Biomedical Image Computing for Neglected Diseases',
    'Special Topic -> Telemedicine and Mobile Health Imaging Technologies'
  ];

  // Mobile breakpoints
  const { isMobile } = useMobile();

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

  // Filter search results based on selected subject areas
  const filteredSearchResults = useMemo(() => {
    if (selectedSubjectAreas.length === 0) {
      return searchResults;
    }

    return searchResults.filter(paper =>
      paper.subject_areas.some(area => selectedSubjectAreas.includes(area))
    );
  }, [searchResults, selectedSubjectAreas]);

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

  // Mobile-specific handlers
  const handleMobileMenuToggle = useCallback(() => {
    setIsMobileMenuOpen(prev => !prev);
  }, []);

  const handleMobileMenuClose = useCallback(() => {
    setIsMobileMenuOpen(false);
  }, []);

  const handleSearchFocus = useCallback(() => {
    setIsSearchFocused(true);
  }, []);

  const handleSearchBlur = useCallback(() => {
    setIsSearchFocused(false);
  }, []);

  const handleInfoToggle = useCallback(() => {
    setIsInfoPopupOpen(prev => !prev);
  }, []);

  const handleInfoClose = useCallback(() => {
    setIsInfoPopupOpen(false);
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
    <div className="App app-layout" style={{
      fontFamily: 'system-ui, -apple-system, sans-serif',
      height: '100vh',
      display: 'flex',
      flexDirection: 'column',
      backgroundColor: 'var(--color-bg-primary)',
      color: 'var(--color-text-primary)'
    }}>
      {/* Mobile Warning */}
      <MobileWarning />

      {/* Header */}
      <header className="app-header" style={{
        padding: isMobile ? '16px' : '20px',
        backgroundColor: 'var(--color-bg-card)',
        borderBottom: '2px solid var(--color-border)',
        boxShadow: 'var(--shadow-md)',
        minHeight: isMobile ? '60px' : 'auto'
      }}>
        <div style={{
          maxWidth: '1200px',
          margin: '0 auto',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          gap: '16px'
        }}>
          <div style={{ flex: 1, minWidth: 0 }}>
            <h1 style={{
              margin: '0 0 8px 0',
              fontSize: isMobile ? '20px' : '28px',
              color: 'var(--color-text-primary)',
              lineHeight: 1.2
            }}>
              MICCAI 2025 Papers Visualization
            </h1>
            {!isMobile && (
              <p style={{
                margin: '0',
                fontSize: '16px',
                color: 'var(--color-text-secondary)'
              }}>
                Interactive t-SNE visualization for exploring conference papers
              </p>
            )}
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
            {isMobile && (
              <HamburgerMenu
                isOpen={isMobileMenuOpen}
                onToggle={handleMobileMenuToggle}
              />
            )}
            <button
              className="info-icon-button"
              onClick={handleInfoToggle}
              aria-label="About this project"
              title="About this project"
            >
              ℹ️
            </button>
            <ThemeToggle />
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="app-main" style={{
        flex: 1,
        display: 'flex',
        flexDirection: isMobile ? 'column' : 'row',
        padding: isMobile ? '8px' : '10px',
        gap: isMobile ? '8px' : '10px',
        height: isMobile ? 'auto' : 'calc(100vh - 120px)',
        overflow: isMobile ? 'visible' : 'hidden',
        width: '100%',
        boxSizing: 'border-box',
        minHeight: isMobile ? 'calc(100vh - 60px)' : 'auto'
      }}>
        {/* Left Column - Search and Visualization */}
        <div className="app-left-column" style={{
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          minHeight: '100%',
          minWidth: 0,
          order: isMobile ? 2 : 1,
          overflow: isMobile ? 'visible' : 'hidden'
        }}>
          {/* Search Bar */}
          <div className="search-container" style={{
            marginBottom: '20px',
            width: '100%',
            boxSizing: 'border-box'
          }}>
            <input
              type="text"
              placeholder="Search papers by title, author, or subject area..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleSearch(searchQuery)}
              onFocus={handleSearchFocus}
              onBlur={handleSearchBlur}
              className="search-input"
              style={{
                width: '100%',
                padding: isMobile ? '16px' : '12px',
                fontSize: isMobile ? '16px' : '16px', // Prevents zoom on iOS
                border: '1px solid var(--color-border)',
                borderRadius: '6px',
                backgroundColor: 'var(--color-bg-card)',
                color: 'var(--color-text-primary)',
                minHeight: isMobile ? '44px' : 'auto'
              }}
            />
            <button
              onClick={() => handleSearch(searchQuery)}
              disabled={isSearching}
              className="search-button"
              style={{
                marginTop: '10px',
                padding: isMobile ? '12px 20px' : '10px 20px',
                backgroundColor: 'var(--color-accent)',
                color: 'white',
                border: 'none',
                borderRadius: '6px',
                cursor: 'pointer',
                fontSize: '14px',
                minHeight: isMobile ? '44px' : 'auto',
                width: isMobile ? '100%' : 'auto'
              }}
            >
              {isSearching ? 'Searching...' : 'Search'}
            </button>
          </div>

          {/* Subject Area Filter */}
          <div style={{
            marginBottom: '20px',
            width: '100%',
            boxSizing: 'border-box'
          }}>
            <SubjectAreaFilter
              selectedAreas={selectedSubjectAreas}
              onSelectionChange={setSelectedSubjectAreas}
              availableAreas={availableSubjectAreas}
              isMobile={isMobile}
            />
          </div>

          {/* Search Results */}
          {filteredSearchResults.length > 0 && (
            <div className="search-results" style={{
              marginBottom: '20px',
              maxHeight: isMobile ? '300px' : '200px',
              overflowY: 'auto'
            }}>
              <h3 style={{ margin: '0 0 10px 0', fontSize: '16px' }}>
                Search Results ({filteredSearchResults.length}
                {selectedSubjectAreas.length > 0 && ` of ${searchResults.length}`})
              </h3>
              {filteredSearchResults.map((paper) => (
                <div
                  key={paper.id}
                  onClick={() => handlePaperSelect(paper)}
                  className="paper-card"
                  style={{
                    padding: isMobile ? '16px' : '10px',
                    marginBottom: '5px',
                    backgroundColor: 'var(--color-bg-card)',
                    border: '1px solid var(--color-border)',
                    borderRadius: '4px',
                    cursor: 'pointer',
                    fontSize: '14px',
                    minHeight: isMobile ? '44px' : 'auto'
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
          <div className="graph-container" style={{
            flex: isMobile ? 'none' : 1,
            overflow: 'hidden',
            position: 'relative',
            minHeight: isMobile ? '400px' : '600px',
            height: isMobile ? '400px' : 'auto'
          }}>
            <TSNEClusterView
              width={isMobile ? window.innerWidth - 32 : Math.max(400, window.innerWidth - sidebarWidth - 40)}
              height={isMobile ? 400 : window.innerHeight - 200}
              onNodeClick={handleNodeClick}
              selectedPaperId={selectedPaper?.id}
              selectedSubjectAreas={selectedSubjectAreas}
            />
          </div>
        </div>

        {/* Resize Handle - Hidden on mobile */}
        {!isMobile && (
          <div
            className="resize-handle"
            onMouseDown={handleMouseDown}
            style={{
              width: '4px',
              backgroundColor: isResizing ? 'var(--color-accent)' : 'var(--color-border)',
              cursor: 'col-resize',
              flexShrink: 0,
              transition: 'background-color var(--transition-fast)'
            }}
          />
        )}

        {/* Right Column - Paper Details */}
        <div className="app-right-column" style={{
          width: isMobile ? '100%' : `${Math.min(sidebarWidth, window.innerWidth * 0.6)}px`,
          flexShrink: 0,
          display: 'flex',
          flexDirection: 'column',
          maxWidth: isMobile ? 'none' : '60vw',
          height: isMobile ? 'auto' : 'auto',
          minHeight: isMobile ? '200px' : 'auto',
          order: isMobile ? 1 : 2,
          overflow: isMobile ? 'visible' : 'auto'
        }}>
          {selectedPaper ? (
            <>
              <PaperDetailsPanel
                paper={selectedPaper}
                isOpen={true}
                onClose={handleCloseDetailsPanel}
                isPersistent={true}
                isExpanded={isPaperDetailsExpanded}
                onToggleExpanded={() => setIsPaperDetailsExpanded(!isPaperDetailsExpanded)}
              />
              <SimilarPapersSection
                selectedPaper={selectedPaper}
                onPaperSelect={handlePaperSelect}
                isMobile={isMobile}
                isExpanded={isSimilarPapersExpanded}
                onToggleExpanded={() => setIsSimilarPapersExpanded(!isSimilarPapersExpanded)}
              />
            </>
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

      {/* Mobile Sidebar */}
      {isMobile && (
        <MobileSidebar
          isOpen={isMobileMenuOpen}
          onClose={handleMobileMenuClose}
        >
          <div style={{ marginBottom: '20px' }}>
            <h4 style={{ margin: '0 0 10px 0', fontSize: '16px' }}>Quick Search</h4>
            <input
              type="text"
              placeholder="Search papers..."
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
                color: 'var(--color-text-primary)',
                marginBottom: '10px'
              }}
            />
            <button
              onClick={() => {
                handleSearch(searchQuery);
                handleMobileMenuClose();
              }}
              disabled={isSearching}
              style={{
                width: '100%',
                padding: '12px',
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

          {searchResults.length > 0 && (
            <div>
              <h4 style={{ margin: '0 0 10px 0', fontSize: '16px' }}>
                Search Results ({searchResults.length})
              </h4>
              <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
                {searchResults.map((paper) => (
                  <div
                    key={paper.id}
                    onClick={() => {
                      handlePaperSelect(paper);
                      handleMobileMenuClose();
                    }}
                    style={{
                      padding: '12px',
                      marginBottom: '8px',
                      backgroundColor: 'var(--color-bg-card)',
                      border: '1px solid var(--color-border)',
                      borderRadius: '6px',
                      cursor: 'pointer',
                      fontSize: '14px'
                    }}
                  >
                    <div style={{ fontWeight: 'bold', marginBottom: '4px' }}>
                      {paper.title}
                    </div>
                    <div style={{ color: 'var(--color-text-secondary)', fontSize: '12px' }}>
                      {paper.authors.map(a => a.name).join(', ')}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </MobileSidebar>
      )}

      {/* Floating Action Button for Search */}
      {isMobile && !isSearchFocused && (
        <FloatingActionButton
          onClick={handleMobileMenuToggle}
          icon="🔍"
          label="Open search"
        />
      )}

      {/* Info Popup */}
      <InfoPopup
        isOpen={isInfoPopupOpen}
        onClose={handleInfoClose}
      />
    </div>
  );
}

export default App;
