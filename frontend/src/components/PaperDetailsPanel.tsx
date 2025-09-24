import React from 'react';
import { Paper } from '../types/api';

interface PaperDetailsPanelProps {
  paper?: Paper | null;
  isOpen: boolean;
  onClose: () => void;
  isPersistent?: boolean;
  isExpanded?: boolean;
  onToggleExpanded?: () => void;
}

const PaperDetailsPanel: React.FC<PaperDetailsPanelProps> = ({
  paper,
  isOpen,
  onClose,
  isPersistent = false,
  isExpanded = true,
  onToggleExpanded
}) => {
  if (!isOpen || !paper) {
    return null;
  }

  return (
    <div style={{
      ...(isPersistent ? {
        backgroundColor: 'var(--color-bg-card)',
        border: '1px solid var(--color-border)',
        borderRadius: '8px',
        overflowY: 'auto',
        flex: '0 0 auto'
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
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            {onToggleExpanded && (
              <button
                onClick={onToggleExpanded}
                style={{
                  background: 'none',
                  border: 'none',
                  fontSize: '14px',
                  cursor: 'pointer',
                  padding: '6px',
                  color: 'var(--color-text-muted)',
                  transition: 'color var(--transition-fast)',
                  borderRadius: '4px',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  width: '24px',
                  height: '24px'
                }}
                title={isExpanded ? 'Collapse' : 'Expand'}
                onMouseEnter={(e) => e.currentTarget.style.color = 'var(--color-text-primary)'}
                onMouseLeave={(e) => e.currentTarget.style.color = 'var(--color-text-muted)'}
              >
                {isExpanded ? '▼' : '▶'}
              </button>
            )}
            <button
              onClick={onClose}
              style={{
                background: 'none',
                border: 'none',
                fontSize: '24px',
                cursor: 'pointer',
                padding: '0',
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
      </div>

      {/* Content */}
      {isExpanded && (
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
        </div>
      )}
    </div>
  );
};

export default PaperDetailsPanel;
