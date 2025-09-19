import React, { useState, useEffect } from 'react';
import { Paper, PaperSimilarity } from '../types/api';
import { apiService } from '../services/api';

interface PaperDetailsPanelProps {
  paper?: Paper | null;
  isOpen: boolean;
  onClose: () => void;
  onPaperSelect?: (paper: Paper) => void;
  isPersistent?: boolean;
}

const PaperDetailsPanel: React.FC<PaperDetailsPanelProps> = ({
  paper,
  isOpen,
  onClose,
  onPaperSelect,
  isPersistent = false
}) => {
  const [similarPapers, setSimilarPapers] = useState<PaperSimilarity[]>([]);
  const [isLoadingSimilar, setIsLoadingSimilar] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (paper && isOpen) {
      setIsLoadingSimilar(true);
      setError(null);

      apiService.getSimilarPapers(paper.id, 5)
        .then(setSimilarPapers)
        .catch(err => {
          setError('Failed to load similar papers');
          console.error('Error loading similar papers:', err);
        })
        .finally(() => setIsLoadingSimilar(false));
    }
  }, [paper, isOpen]);

  if (!isOpen || !paper) {
    return null;
  }

  return (
    <div style={{
      ...(isPersistent ? {
        height: '100%',
        backgroundColor: 'var(--color-bg-card)',
        border: '1px solid var(--color-border)',
        borderRadius: '8px',
        overflowY: 'auto'
      } : {
        position: 'fixed',
        top: 0,
        right: 0,
        width: '400px',
        height: '100vh',
        backgroundColor: 'var(--color-bg-card)',
        boxShadow: 'var(--shadow-lg)',
        zIndex: 1000,
        overflowY: 'auto',
        padding: 0,
        borderLeft: '1px solid var(--color-border)'
      })
    }}>
      {/* Header */}
      <div style={{
        padding: '20px',
        borderBottom: '1px solid var(--color-border)',
        backgroundColor: 'var(--color-bg-secondary)',
        position: 'sticky',
        top: 0,
        zIndex: 1001
      }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
          <h3 style={{ margin: 0, fontSize: '18px', lineHeight: 1.3, color: 'var(--color-text-primary)' }}>
            Paper Details
          </h3>
          <button
            onClick={onClose}
            style={{
              background: 'none',
              border: 'none',
              fontSize: '24px',
              cursor: 'pointer',
              padding: '0',
              marginLeft: '10px',
              color: 'var(--color-text-muted)',
              transition: 'color var(--transition-fast)'
            }}
            title="Close"
            onMouseEnter={(e) => e.currentTarget.style.color = 'var(--color-text-primary)'}
            onMouseLeave={(e) => e.currentTarget.style.color = 'var(--color-text-muted)'}
          >
            ×
          </button>
        </div>
      </div>

      {/* Content */}
      <div style={{ padding: '20px' }}>
        {/* Title */}
        <div style={{ marginBottom: '15px' }}>
          <h2 style={{
            margin: '0 0 10px 0',
            fontSize: '20px',
            lineHeight: 1.3,
            color: 'var(--color-text-primary)'
          }}>
            {paper.title}
          </h2>

          <div style={{ fontSize: '14px', color: 'var(--color-text-secondary)', marginBottom: '10px' }}>
            Paper ID: {paper.id}
          </div>
        </div>

        {/* Authors */}
        <div style={{ marginBottom: '20px' }}>
          <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
            Authors ({paper.authors.length})
          </h4>
          <div style={{ fontSize: '14px' }}>
            {paper.authors.map((author, index) => (
              <div key={index} style={{ marginBottom: '5px' }}>
                <strong style={{ color: 'var(--color-text-primary)' }}>{author.name}</strong>
                {author.affiliation && (
                  <div style={{ fontSize: '12px', color: 'var(--color-text-secondary)' }}>
                    {author.affiliation}
                  </div>
                )}
                {author.email && (
                  <div style={{ fontSize: '12px', color: 'var(--color-accent)' }}>
                    {author.email}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Subject Areas */}
        {paper.subject_areas.length > 0 && (
          <div style={{ marginBottom: '20px' }}>
            <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
              Subject Areas
            </h4>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '5px' }}>
              {paper.subject_areas.map(area => (
                <span
                  key={area}
                  style={{
                    padding: '4px 8px',
                    backgroundColor: 'var(--color-bg-hover)',
                    borderRadius: '12px',
                    fontSize: '12px',
                    color: 'var(--color-text-secondary)'
                  }}
                >
                  {area}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Abstract */}
        <div style={{ marginBottom: '20px' }}>
          <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
            Abstract
          </h4>
          <p style={{
            fontSize: '14px',
            lineHeight: 1.5,
            margin: 0,
            textAlign: 'justify',
            color: 'var(--color-text-secondary)'
          }}>
            {paper.abstract}
          </p>
        </div>

        {/* Publication Info */}
        {paper.publication_date && (
          <div style={{ marginBottom: '20px' }}>
            <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
              Publication Date
            </h4>
            <p style={{ fontSize: '14px', margin: 0, color: 'var(--color-text-secondary)' }}>
              {new Date(paper.publication_date).toLocaleDateString()}
            </p>
          </div>
        )}

        {/* External Links */}
        {paper.external_links.length > 0 && (
          <div style={{ marginBottom: '20px' }}>
            <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
              Links
            </h4>
            {paper.external_links.map((link, index) => (
              <div key={index} style={{ marginBottom: '5px' }}>
                <a
                  href={link.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  style={{
                    color: 'var(--color-accent)',
                    textDecoration: 'none',
                    fontSize: '14px',
                    transition: 'color var(--transition-fast)'
                  }}
                  onMouseEnter={(e) => e.currentTarget.style.color = 'var(--color-accent-hover)'}
                  onMouseLeave={(e) => e.currentTarget.style.color = 'var(--color-accent)'}
                >
                  {link.description || link.type.toUpperCase()} ↗
                </a>
              </div>
            ))}
          </div>
        )}

        {/* Similar Papers */}
        <div style={{ marginBottom: '20px' }}>
          <h4 style={{ margin: '0 0 12px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
            Similar Papers
          </h4>

          {isLoadingSimilar && (
            <div style={{ textAlign: 'center', padding: '20px', color: 'var(--color-text-secondary)' }}>
              Loading similar papers...
            </div>
          )}

          {error && (
            <div style={{
              padding: '10px',
              backgroundColor: 'var(--color-error)',
              color: 'white',
              borderRadius: '4px',
              fontSize: '14px'
            }}>
              {error}
            </div>
          )}

          {!isLoadingSimilar && !error && similarPapers.length === 0 && (
            <div style={{ color: 'var(--color-text-secondary)', fontSize: '14px' }}>
              No similar papers found.
            </div>
          )}

          {similarPapers.length > 0 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              {similarPapers.map((similar) => (
                <div
                  key={similar.paper.id}
                  onClick={() => onPaperSelect?.(similar.paper)}
                  style={{
                    padding: '12px',
                    backgroundColor: 'var(--color-bg-hover)',
                    borderRadius: '6px',
                    cursor: 'pointer',
                    transition: 'background-color var(--transition-fast)',
                    border: '1px solid var(--color-border)'
                  }}
                  onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--color-bg-secondary)'}
                  onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)'}
                >
                  <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'flex-start',
                    marginBottom: '6px'
                  }}>
                    <h5 style={{
                      margin: '0',
                      fontSize: '13px',
                      lineHeight: 1.3,
                      color: 'var(--color-text-primary)',
                      fontWeight: '600'
                    }}>
                      {similar.paper.title.length > 80
                        ? similar.paper.title.substring(0, 80) + '...'
                        : similar.paper.title}
                    </h5>
                    <span style={{
                      fontSize: '11px',
                      color: 'var(--color-accent)',
                      marginLeft: '8px',
                      flexShrink: 0,
                      fontWeight: '500'
                    }}>
                      {(similar.similarity_score * 100).toFixed(1)}%
                    </span>
                  </div>
                  <div style={{ fontSize: '12px', color: 'var(--color-text-secondary)', marginBottom: '4px' }}>
                    {similar.paper.authors.map(a => a.name).join(', ')}
                  </div>
                  {similar.paper.subject_areas.length > 0 && (
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
                      {similar.paper.subject_areas.slice(0, 3).map(area => (
                        <span
                          key={area}
                          style={{
                            padding: '2px 6px',
                            backgroundColor: 'var(--color-bg-primary)',
                            borderRadius: '8px',
                            fontSize: '10px',
                            color: 'var(--color-text-secondary)'
                          }}
                        >
                          {area}
                        </span>
                      ))}
                      {similar.paper.subject_areas.length > 3 && (
                        <span style={{ fontSize: '10px', color: 'var(--color-text-muted)' }}>
                          +{similar.paper.subject_areas.length - 3} more
                        </span>
                      )}
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default PaperDetailsPanel;
