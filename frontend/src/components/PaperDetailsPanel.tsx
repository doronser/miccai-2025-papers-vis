import React, { useState, useEffect } from 'react';
import { Paper, PaperSimilarity } from '../types/api';
import ApiService from '../services/api';

interface PaperDetailsPanelProps {
  paper?: Paper | null;
  isOpen: boolean;
  onClose: () => void;
  onPaperSelect?: (paper: Paper) => void;
  favorites?: Set<string>;
  onToggleFavorite?: (paperId: string) => void;
  isPersistent?: boolean;
}

const PaperDetailsPanel: React.FC<PaperDetailsPanelProps> = ({
  paper,
  isOpen,
  onClose,
  onPaperSelect,
  favorites = new Set(),
  onToggleFavorite,
  isPersistent = false
}) => {
  const [similarPapers, setSimilarPapers] = useState<PaperSimilarity[]>([]);
  const [isLoadingSimilar, setIsLoadingSimilar] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (paper && isOpen) {
      setIsLoadingSimilar(true);
      setError(null);

      ApiService.getSimilarPapers(paper.id, 5)
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

  const isFavorite = favorites.has(paper.id);

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
        {/* Title and Favorite Button */}
        <div style={{ marginBottom: '15px' }}>
          <div style={{ display: 'flex', alignItems: 'flex-start', gap: '10px' }}>
            <h2 style={{
              margin: '0 0 10px 0',
              fontSize: '20px',
              lineHeight: 1.3,
              color: 'var(--color-text-primary)',
              flex: 1
            }}>
              {paper.title}
            </h2>
            <button
              onClick={() => onToggleFavorite?.(paper.id)}
              style={{
                background: 'none',
                border: 'none',
                fontSize: '24px',
                cursor: 'pointer',
                padding: '5px',
                color: isFavorite ? 'var(--color-error)' : 'var(--color-text-muted)',
                transition: 'color var(--transition-fast)'
              }}
              title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
            >
              {isFavorite ? '♥' : '♡'}
            </button>
          </div>

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
          <h4 style={{ margin: '0 0 8px 0', fontSize: '16px', color: 'var(--color-text-primary)' }}>
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
            <div style={{ fontSize: '14px' }}>
              {similarPapers.map((similar) => (
                <div
                  key={similar.paper.id}
                  style={{
                    padding: '10px',
                    marginBottom: '8px',
                    backgroundColor: 'var(--color-bg-hover)',
                    borderRadius: '6px',
                    cursor: 'pointer',
                    transition: 'background-color var(--transition-fast)'
                  }}
                  onClick={() => onPaperSelect?.(similar.paper)}
                  onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--color-bg-secondary)'}
                  onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)'}
                >
                  <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'flex-start',
                    marginBottom: '5px'
                  }}>
                    <strong style={{ fontSize: '13px', lineHeight: 1.3, color: 'var(--color-text-primary)' }}>
                      {similar.paper.title.length > 60
                        ? similar.paper.title.substring(0, 60) + '...'
                        : similar.paper.title}
                    </strong>
                    <span style={{
                      fontSize: '11px',
                      color: 'var(--color-accent)',
                      marginLeft: '8px',
                      flexShrink: 0
                    }}>
                      {(similar.similarity_score * 100).toFixed(1)}%
                    </span>
                  </div>
                  <div style={{ fontSize: '12px', color: 'var(--color-text-secondary)' }}>
                    {similar.paper.authors.map(a => a.name).join(', ')}
                  </div>
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
