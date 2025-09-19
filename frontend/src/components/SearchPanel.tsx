import React, { useState, useCallback, useEffect, useRef } from 'react';
import { Paper } from '../types/api';

interface SearchPanelProps {
  onSearch: (query: string) => void;
  onFilterChange?: (filters: SearchFilters) => void;
  searchResults?: Paper[];
  isSearching?: boolean;
  onPaperSelect?: (paper: Paper) => void;
  selectedPaperId?: string | null;
}

interface SearchFilters {
  authors?: string[];
  subjectAreas?: string[];
  dateRange?: [string, string];
}

const SearchPanel: React.FC<SearchPanelProps> = ({
  onSearch,
  onFilterChange,
  searchResults = [],
  isSearching = false,
  onPaperSelect,
  selectedPaperId
}) => {
  const [query, setQuery] = useState('');
  const [selectedAuthors, setSelectedAuthors] = useState<string[]>([]);
  const [selectedSubjectAreas, setSelectedSubjectAreas] = useState<string[]>([]);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Debounced search effect
  useEffect(() => {
    if (debounceTimeoutRef.current) {
      clearTimeout(debounceTimeoutRef.current);
    }

    if (query.trim()) {
      debounceTimeoutRef.current = setTimeout(() => {
        onSearch(query.trim());
      }, 300); // 300ms debounce delay
    }

    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
    };
  }, [query, onSearch]);

  const handleSearch = useCallback((e: React.FormEvent) => {
    e.preventDefault();
    if (query.trim()) {
      onSearch(query.trim());
    }
  }, [query, onSearch]);

  const handleKeyPress = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (query.trim()) {
        onSearch(query.trim());
      }
    }
  }, [query, onSearch]);

  const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value);
  }, []);

  // Extract unique authors and subject areas from search results
  const uniqueAuthors = React.useMemo(() => {
    const authors = new Set<string>();
    searchResults.forEach(paper => {
      paper.authors.forEach(author => authors.add(author.name));
    });
    return Array.from(authors).sort();
  }, [searchResults]);

  const uniqueSubjectAreas = React.useMemo(() => {
    const areas = new Set<string>();
    searchResults.forEach(paper => {
      paper.subject_areas.forEach(area => areas.add(area));
    });
    return Array.from(areas).sort();
  }, [searchResults]);

  return (
    <div className="search-panel card" style={{
      padding: '20px',
      marginBottom: '20px'
    }}>
      {/* Main Search */}
      <form onSubmit={handleSearch} style={{ marginBottom: '15px' }}>
        <div style={{ display: 'flex', gap: '10px' }}>
          <input
            type="text"
            value={query}
            onChange={handleInputChange}
            onKeyPress={handleKeyPress}
            placeholder="Search papers by title, abstract, authors, or subject areas..."
            style={{
              flex: 1,
              padding: '12px',
              borderRadius: '6px',
              border: '1px solid var(--color-border)',
              fontSize: '16px',
              backgroundColor: 'var(--color-bg-card)',
              color: 'var(--color-text-primary)',
              transition: 'border-color var(--transition-fast)'
            }}
          />
          <button
            type="submit"
            disabled={!query.trim() || isSearching}
            className="btn btn-primary"
            style={{
              padding: '12px 24px',
              fontSize: '16px',
              minWidth: '100px'
            }}
          >
            {isSearching ? 'Searching...' : 'Search'}
          </button>
        </div>
      </form>

      {/* Advanced Filters Toggle */}
      <button
        onClick={() => setShowAdvanced(!showAdvanced)}
        className="btn"
        style={{
          marginBottom: '15px'
        }}
      >
        {showAdvanced ? 'Hide' : 'Show'} Advanced Filters
      </button>

      {/* Advanced Filters */}
      {showAdvanced && (
        <div className="advanced-filters card" style={{
          padding: '15px',
          marginBottom: '15px'
        }}>
          <h4 style={{ margin: '0 0 15px 0', color: 'var(--color-text-primary)' }}>Advanced Filters</h4>

          {/* Authors Filter */}
          {uniqueAuthors.length > 0 && (
            <div style={{ marginBottom: '15px' }}>
              <label style={{ display: 'block', marginBottom: '5px', fontWeight: 'bold', color: 'var(--color-text-primary)' }}>
                Authors ({uniqueAuthors.length})
              </label>
              <select
                multiple
                value={selectedAuthors}
                onChange={(e) => {
                  const values = Array.from(e.target.selectedOptions, option => option.value);
                  setSelectedAuthors(values);
                  onFilterChange?.({ authors: values, subjectAreas: selectedSubjectAreas });
                }}
                style={{
                  width: '100%',
                  minHeight: '80px',
                  padding: '5px',
                  borderRadius: '4px',
                  border: '1px solid var(--color-border)',
                  backgroundColor: 'var(--color-bg-card)',
                  color: 'var(--color-text-primary)'
                }}
              >
                {uniqueAuthors.map(author => (
                  <option key={author} value={author}>
                    {author}
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Subject Areas Filter */}
          {uniqueSubjectAreas.length > 0 && (
            <div style={{ marginBottom: '15px' }}>
              <label style={{ display: 'block', marginBottom: '5px', fontWeight: 'bold', color: 'var(--color-text-primary)' }}>
                Subject Areas ({uniqueSubjectAreas.length})
              </label>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '5px' }}>
                {uniqueSubjectAreas.map(area => (
                  <label key={area} style={{
                    display: 'flex',
                    alignItems: 'center',
                    padding: '5px 10px',
                    backgroundColor: selectedSubjectAreas.includes(area) ? 'var(--color-accent)' : 'var(--color-bg-hover)',
                    color: selectedSubjectAreas.includes(area) ? 'white' : 'var(--color-text-primary)',
                    borderRadius: '20px',
                    cursor: 'pointer',
                    fontSize: '12px',
                    transition: 'all var(--transition-fast)'
                  }}>
                    <input
                      type="checkbox"
                      checked={selectedSubjectAreas.includes(area)}
                      onChange={(e) => {
                        const newAreas = e.target.checked
                          ? [...selectedSubjectAreas, area]
                          : selectedSubjectAreas.filter(a => a !== area);
                        setSelectedSubjectAreas(newAreas);
                        onFilterChange?.({ authors: selectedAuthors, subjectAreas: newAreas });
                      }}
                      style={{ display: 'none' }}
                    />
                    {area}
                  </label>
                ))}
              </div>
            </div>
          )}

          {/* Clear Filters */}
          {(selectedAuthors.length > 0 || selectedSubjectAreas.length > 0) && (
            <button
              onClick={() => {
                setSelectedAuthors([]);
                setSelectedSubjectAreas([]);
                onFilterChange?.({ authors: [], subjectAreas: [] });
              }}
              className="btn"
              style={{
                backgroundColor: 'var(--color-text-muted)',
                color: 'white'
              }}
            >
              Clear All Filters
            </button>
          )}
        </div>
      )}

      {/* Search Results */}
      {searchResults.length > 0 && (
        <div className="search-results card" style={{
          maxHeight: '400px',
          overflowY: 'auto'
        }}>
          <h4 style={{ padding: '15px 15px 10px', margin: 0, borderBottom: '1px solid var(--color-border)', color: 'var(--color-text-primary)' }}>
            Search Results ({searchResults.length})
          </h4>
          {searchResults.map(paper => {
            const isSelected = selectedPaperId === paper.id;
            return (
              <div
                key={paper.id}
                onClick={() => onPaperSelect?.(paper)}
                style={{
                  padding: '15px',
                  borderBottom: '1px solid var(--color-border)',
                  cursor: 'pointer',
                  transition: 'background-color var(--transition-fast)',
                  backgroundColor: isSelected ? 'var(--color-accent)' : 'var(--color-bg-card)',
                  borderLeft: isSelected ? '4px solid var(--color-accent)' : '4px solid transparent'
                }}
                onMouseEnter={(e) => {
                  if (!isSelected) {
                    e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)';
                  }
                }}
                onMouseLeave={(e) => {
                  if (!isSelected) {
                    e.currentTarget.style.backgroundColor = 'var(--color-bg-card)';
                  }
                }}
              >
                <h5 style={{
                  margin: '0 0 5px 0',
                  color: isSelected ? 'white' : 'var(--color-accent)'
                }}>
                  {paper.title}
                  {isSelected && <span style={{ marginLeft: '8px' }}>📍</span>}
                </h5>
                <p style={{
                  margin: '0 0 8px 0',
                  fontSize: '14px',
                  color: isSelected ? 'rgba(255,255,255,0.9)' : 'var(--color-text-secondary)'
                }}>
                  {paper.authors.map(a => a.name).join(', ')}
                </p>
                <p style={{
                  margin: '0 0 5px 0',
                  fontSize: '13px',
                  color: isSelected ? 'rgba(255,255,255,0.8)' : 'var(--color-text-muted)'
                }}>
                  {paper.abstract.length > 200
                    ? paper.abstract.substring(0, 200) + '...'
                    : paper.abstract}
                </p>
                {paper.subject_areas.length > 0 && (
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px' }}>
                    {paper.subject_areas.map(area => (
                      <span
                        key={area}
                        style={{
                          padding: '2px 6px',
                          backgroundColor: isSelected ? 'rgba(255,255,255,0.2)' : 'var(--color-bg-hover)',
                          borderRadius: '10px',
                          fontSize: '11px',
                          color: isSelected ? 'white' : 'var(--color-text-secondary)'
                        }}
                      >
                        {area}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}

      {/* No Results */}
      {query.trim() && searchResults.length === 0 && !isSearching && (
        <div className="card" style={{
          padding: '20px',
          textAlign: 'center',
          color: 'var(--color-text-secondary)'
        }}>
          No papers found for "{query}". Try different keywords or check your spelling.
        </div>
      )}
    </div>
  );
};

export default SearchPanel;
