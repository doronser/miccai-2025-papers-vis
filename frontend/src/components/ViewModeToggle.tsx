import React from 'react';
import { ViewMode } from '../types/api';

interface ViewModeToggleProps {
    currentMode: ViewMode;
    onModeChange: (mode: ViewMode) => void;
}

export const ViewModeToggle: React.FC<ViewModeToggleProps> = ({
    currentMode,
    onModeChange
}) => {
    return (
        <div className="view-mode-toggle" style={{
            display: 'flex',
            gap: '4px',
            padding: '4px',
            background: 'var(--color-bg-secondary)',
            borderRadius: '8px',
            border: '1px solid var(--color-border)'
        }}>
            <button
                onClick={() => onModeChange('clusters')}
                className={currentMode === 'clusters' ? 'active' : ''}
                style={{
                    padding: '8px 16px',
                    border: 'none',
                    borderRadius: '6px',
                    background: currentMode === 'clusters' ? 'var(--color-accent)' : 'transparent',
                    color: currentMode === 'clusters' ? 'white' : 'var(--color-text-secondary)',
                    cursor: 'pointer',
                    transition: 'all var(--transition-fast)',
                    fontSize: '14px',
                    fontWeight: currentMode === 'clusters' ? '600' : '400'
                }}
                title="Cluster View - Papers grouped by topics"
            >
                📊 Clusters
            </button>
            <button
                onClick={() => onModeChange('similarity')}
                className={currentMode === 'similarity' ? 'active' : ''}
                style={{
                    padding: '8px 16px',
                    border: 'none',
                    borderRadius: '6px',
                    background: currentMode === 'similarity' ? 'var(--color-accent)' : 'transparent',
                    color: currentMode === 'similarity' ? 'white' : 'var(--color-text-secondary)',
                    cursor: 'pointer',
                    transition: 'all var(--transition-fast)',
                    fontSize: '14px',
                    fontWeight: currentMode === 'similarity' ? '600' : '400'
                }}
                title="Similarity Network - Papers connected by similarity"
            >
                🔗 Network
            </button>
        </div>
    );
};
