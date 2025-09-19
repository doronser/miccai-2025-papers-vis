import React, { useState, useMemo } from 'react';

interface SubjectAreaFilterProps {
    availableAreas: string[];
    selectedAreas: string[];
    onSelectionChange: (areas: string[]) => void;
    isLoading?: boolean;
}

const SubjectAreaFilter: React.FC<SubjectAreaFilterProps> = ({
    availableAreas,
    selectedAreas,
    onSelectionChange,
    isLoading = false
}) => {
    const [isOpen, setIsOpen] = useState(false);
    const [searchTerm, setSearchTerm] = useState('');

    // Filter areas based on search term
    const filteredAreas = useMemo(() => {
        if (!searchTerm) return availableAreas;
        return availableAreas.filter(area =>
            area.toLowerCase().includes(searchTerm.toLowerCase())
        );
    }, [availableAreas, searchTerm]);

    const handleAreaToggle = (area: string) => {
        const newSelection = selectedAreas.includes(area)
            ? selectedAreas.filter(a => a !== area)
            : [...selectedAreas, area];
        onSelectionChange(newSelection);
    };

    const handleSelectAll = () => {
        onSelectionChange(filteredAreas);
    };

    const handleClearAll = () => {
        onSelectionChange([]);
    };

    const handleClearSelected = () => {
        onSelectionChange([]);
    };

    return (
        <div className="subject-area-filter" style={{ position: 'relative' }}>
            {/* Filter Button */}
            <button
                onClick={() => setIsOpen(!isOpen)}
                disabled={isLoading}
                className="btn"
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    padding: '8px 16px',
                    fontSize: '14px',
                    minWidth: '200px',
                    justifyContent: 'space-between'
                }}
            >
                <span>
                    {selectedAreas.length === 0
                        ? 'All Subject Areas'
                        : `${selectedAreas.length} area${selectedAreas.length === 1 ? '' : 's'} selected`
                    }
                </span>
                <span style={{
                    transform: isOpen ? 'rotate(180deg)' : 'rotate(0deg)',
                    transition: 'transform var(--transition-fast)'
                }}>
                    ▼
                </span>
            </button>

            {/* Dropdown */}
            {isOpen && (
                <div style={{
                    position: 'absolute',
                    top: '100%',
                    left: 0,
                    right: 0,
                    background: 'var(--color-bg-card)',
                    border: '1px solid var(--color-border)',
                    borderRadius: '8px',
                    boxShadow: 'var(--shadow-lg)',
                    zIndex: 1000,
                    marginTop: '4px',
                    maxHeight: '400px',
                    overflow: 'hidden',
                    display: 'flex',
                    flexDirection: 'column'
                }}>
                    {/* Search Input */}
                    <div style={{ padding: '12px', borderBottom: '1px solid var(--color-border)' }}>
                        <input
                            type="text"
                            placeholder="Search subject areas..."
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                            style={{
                                width: '100%',
                                padding: '8px 12px',
                                border: '1px solid var(--color-border)',
                                borderRadius: '6px',
                                fontSize: '14px',
                                backgroundColor: 'var(--color-bg-card)',
                                color: 'var(--color-text-primary)'
                            }}
                        />
                    </div>

                    {/* Action Buttons */}
                    <div style={{
                        padding: '8px 12px',
                        borderBottom: '1px solid var(--color-border)',
                        display: 'flex',
                        gap: '8px'
                    }}>
                        <button
                            onClick={handleSelectAll}
                            className="btn"
                            style={{ fontSize: '12px', padding: '4px 8px' }}
                        >
                            Select All
                        </button>
                        <button
                            onClick={handleClearAll}
                            className="btn"
                            style={{ fontSize: '12px', padding: '4px 8px' }}
                        >
                            Clear All
                        </button>
                    </div>

                    {/* Areas List */}
                    <div style={{
                        maxHeight: '300px',
                        overflowY: 'auto',
                        padding: '8px 0'
                    }}>
                        {filteredAreas.length === 0 ? (
                            <div style={{
                                padding: '16px',
                                textAlign: 'center',
                                color: 'var(--color-text-muted)',
                                fontSize: '14px'
                            }}>
                                {searchTerm ? 'No areas found matching your search' : 'No subject areas available'}
                            </div>
                        ) : (
                            filteredAreas.map(area => (
                                <label
                                    key={area}
                                    style={{
                                        display: 'flex',
                                        alignItems: 'center',
                                        padding: '8px 16px',
                                        cursor: 'pointer',
                                        transition: 'background-color var(--transition-fast)',
                                        fontSize: '14px'
                                    }}
                                    onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)'}
                                    onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                                >
                                    <input
                                        type="checkbox"
                                        checked={selectedAreas.includes(area)}
                                        onChange={() => handleAreaToggle(area)}
                                        style={{
                                            marginRight: '12px',
                                            accentColor: 'var(--color-accent)'
                                        }}
                                    />
                                    <span style={{ color: 'var(--color-text-primary)' }}>
                                        {area}
                                    </span>
                                </label>
                            ))
                        )}
                    </div>

                    {/* Selected Areas Summary */}
                    {selectedAreas.length > 0 && (
                        <div style={{
                            padding: '12px',
                            borderTop: '1px solid var(--color-border)',
                            backgroundColor: 'var(--color-bg-secondary)'
                        }}>
                            <div style={{
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'center',
                                marginBottom: '8px'
                            }}>
                                <span style={{ fontSize: '12px', color: 'var(--color-text-secondary)' }}>
                                    Selected ({selectedAreas.length}):
                                </span>
                                <button
                                    onClick={handleClearSelected}
                                    className="btn"
                                    style={{ fontSize: '12px', padding: '4px 8px' }}
                                >
                                    Clear
                                </button>
                            </div>
                            <div style={{
                                display: 'flex',
                                flexWrap: 'wrap',
                                gap: '4px'
                            }}>
                                {selectedAreas.slice(0, 3).map(area => (
                                    <span
                                        key={area}
                                        style={{
                                            padding: '2px 6px',
                                            backgroundColor: 'var(--color-accent)',
                                            color: 'white',
                                            borderRadius: '12px',
                                            fontSize: '11px'
                                        }}
                                    >
                                        {area}
                                    </span>
                                ))}
                                {selectedAreas.length > 3 && (
                                    <span style={{
                                        padding: '2px 6px',
                                        backgroundColor: 'var(--color-bg-hover)',
                                        color: 'var(--color-text-secondary)',
                                        borderRadius: '12px',
                                        fontSize: '11px'
                                    }}>
                                        +{selectedAreas.length - 3} more
                                    </span>
                                )}
                            </div>
                        </div>
                    )}
                </div>
            )}

            {/* Click outside to close */}
            {isOpen && (
                <div
                    style={{
                        position: 'fixed',
                        top: 0,
                        left: 0,
                        right: 0,
                        bottom: 0,
                        zIndex: 999
                    }}
                    onClick={() => setIsOpen(false)}
                />
            )}
        </div>
    );
};

export default SubjectAreaFilter;
