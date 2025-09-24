import React, { useState, useEffect } from 'react';
import { Paper, PaperSimilarity } from '../types/api';
import { apiService } from '../services/api';

interface SimilarPapersSectionProps {
    selectedPaper: Paper | null;
    onPaperSelect?: (paper: Paper) => void;
    isMobile?: boolean;
    isExpanded?: boolean;
    onToggleExpanded?: () => void;
}

const SimilarPapersSection: React.FC<SimilarPapersSectionProps> = ({
    selectedPaper,
    onPaperSelect,
    isMobile = false,
    isExpanded = true,
    onToggleExpanded
}) => {
    const [similarPapers, setSimilarPapers] = useState<PaperSimilarity[]>([]);
    const [isLoadingSimilar, setIsLoadingSimilar] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (selectedPaper) {
            setIsLoadingSimilar(true);
            setError(null);

            apiService.getSimilarPapers(selectedPaper.id, 5)
                .then(setSimilarPapers)
                .catch(err => {
                    setError('Failed to load similar papers');
                    console.error('Error loading similar papers:', err);
                })
                .finally(() => setIsLoadingSimilar(false));
        } else {
            setSimilarPapers([]);
            setIsLoadingSimilar(false);
            setError(null);
        }
    }, [selectedPaper]);

    if (!selectedPaper) {
        return null;
    }

    return (
        <div style={{
            padding: isMobile ? '16px' : '20px',
            backgroundColor: 'var(--color-bg-card)',
            border: '1px solid var(--color-border)',
            borderRadius: '8px',
            marginTop: '16px'
        }}>
            <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                marginBottom: '16px'
            }}>
                <h3 style={{
                    margin: '0',
                    fontSize: '18px',
                    color: 'var(--color-text-primary)',
                    fontWeight: '600'
                }}>
                    Similar Papers
                </h3>
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
            </div>

            {isExpanded && (
                <>
                    {isLoadingSimilar && (
                        <div style={{
                            textAlign: 'center',
                            padding: '20px',
                            color: 'var(--color-text-secondary)'
                        }}>
                            Loading similar papers...
                        </div>
                    )}

                    {error && (
                        <div style={{
                            padding: '12px',
                            backgroundColor: 'var(--color-error-light)',
                            color: 'var(--color-error)',
                            borderRadius: '6px',
                            fontSize: '14px',
                            border: '1px solid var(--color-error)'
                        }}>
                            {error}
                        </div>
                    )}

                    {!isLoadingSimilar && !error && similarPapers.length === 0 && (
                        <div style={{
                            color: 'var(--color-text-secondary)',
                            fontSize: '14px',
                            textAlign: 'center',
                            padding: '20px'
                        }}>
                            No similar papers found.
                        </div>
                    )}

                    {similarPapers.length > 0 && (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                            {similarPapers.map((similar) => (
                                <div
                                    key={similar.paper.id}
                                    onClick={() => onPaperSelect?.(similar.paper)}
                                    style={{
                                        padding: '16px',
                                        backgroundColor: 'var(--color-bg-hover)',
                                        borderRadius: '8px',
                                        cursor: 'pointer',
                                        transition: 'all var(--transition-fast)',
                                        border: '1px solid var(--color-border)',
                                        minHeight: isMobile ? '44px' : 'auto'
                                    }}
                                    onMouseEnter={(e) => {
                                        e.currentTarget.style.backgroundColor = 'var(--color-bg-secondary)';
                                        e.currentTarget.style.borderColor = 'var(--color-border-hover)';
                                    }}
                                    onMouseLeave={(e) => {
                                        e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)';
                                        e.currentTarget.style.borderColor = 'var(--color-border)';
                                    }}
                                >
                                    <div style={{
                                        display: 'flex',
                                        justifyContent: 'space-between',
                                        alignItems: 'flex-start',
                                        marginBottom: '8px'
                                    }}>
                                        <h4 style={{
                                            margin: '0',
                                            fontSize: '14px',
                                            lineHeight: 1.4,
                                            color: 'var(--color-text-primary)',
                                            fontWeight: '600',
                                            flex: 1,
                                            marginRight: '12px'
                                        }}>
                                            {similar.paper.title.length > 100
                                                ? similar.paper.title.substring(0, 100) + '...'
                                                : similar.paper.title}
                                        </h4>
                                        <span style={{
                                            fontSize: '12px',
                                            color: 'var(--color-accent)',
                                            flexShrink: 0,
                                            fontWeight: '600',
                                            backgroundColor: 'var(--color-accent-light)',
                                            padding: '2px 8px',
                                            borderRadius: '12px'
                                        }}>
                                            {(similar.similarity_score * 100).toFixed(1)}%
                                        </span>
                                    </div>

                                    <div style={{
                                        fontSize: '13px',
                                        color: 'var(--color-text-secondary)',
                                        marginBottom: '8px',
                                        lineHeight: 1.3
                                    }}>
                                        {similar.paper.authors.map(a => a.name).join(', ')}
                                    </div>

                                    {similar.paper.subject_areas.length > 0 && (
                                        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                                            {similar.paper.subject_areas.slice(0, 3).map(area => (
                                                <span
                                                    key={area}
                                                    style={{
                                                        padding: '3px 8px',
                                                        backgroundColor: 'var(--color-bg-primary)',
                                                        borderRadius: '12px',
                                                        fontSize: '11px',
                                                        color: 'var(--color-text-secondary)',
                                                        border: '1px solid var(--color-border-light)'
                                                    }}
                                                >
                                                    {area}
                                                </span>
                                            ))}
                                            {similar.paper.subject_areas.length > 3 && (
                                                <span style={{
                                                    fontSize: '11px',
                                                    color: 'var(--color-text-muted)',
                                                    alignSelf: 'center'
                                                }}>
                                                    +{similar.paper.subject_areas.length - 3} more
                                                </span>
                                            )}
                                        </div>
                                    )}
                                </div>
                            ))}
                        </div>
                    )}
                </>
            )}
        </div>
    );
};

export default SimilarPapersSection;
