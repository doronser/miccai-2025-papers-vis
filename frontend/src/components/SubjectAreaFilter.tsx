import React, { useState, useRef, useEffect, useMemo } from 'react';

interface SubjectAreaFilterProps {
    selectedAreas: string[];
    onSelectionChange: (areas: string[]) => void;
    availableAreas: string[];
    isMobile?: boolean;
}

const SubjectAreaFilter: React.FC<SubjectAreaFilterProps> = ({
    selectedAreas,
    onSelectionChange,
    availableAreas,
    isMobile = false
}) => {
    const [isOpen, setIsOpen] = useState(false);
    const [searchQuery, setSearchQuery] = useState('');
    const dropdownRef = useRef<HTMLDivElement>(null);
    const searchInputRef = useRef<HTMLInputElement>(null);

    // Filter areas based on search query
    const filteredAreas = useMemo(() => {
        if (!searchQuery.trim()) {
            return availableAreas;
        }

        const query = searchQuery.toLowerCase();
        return availableAreas.filter(area =>
            area.toLowerCase().includes(query)
        );
    }, [availableAreas, searchQuery]);

    // Close dropdown when clicking outside
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
                setIsOpen(false);
                setSearchQuery(''); // Clear search when closing
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    // Focus search input when dropdown opens
    useEffect(() => {
        if (isOpen && searchInputRef.current) {
            searchInputRef.current.focus();
        }
    }, [isOpen]);

    const handleAreaToggle = (area: string) => {
        if (selectedAreas.includes(area)) {
            onSelectionChange(selectedAreas.filter(a => a !== area));
        } else {
            onSelectionChange([...selectedAreas, area]);
        }
    };

    const handleClearAll = () => {
        onSelectionChange([]);
    };

    const handleSelectAll = () => {
        onSelectionChange([...availableAreas]);
    };

    const handleClearSearch = () => {
        setSearchQuery('');
    };

    const getDisplayText = () => {
        if (selectedAreas.length === 0) {
            return 'All Subject Areas';
        } else if (selectedAreas.length === 1) {
            return selectedAreas[0];
        } else if (selectedAreas.length <= 3) {
            return selectedAreas.join(', ');
        } else {
            return `${selectedAreas.length} areas selected`;
        }
    };

    return (
        <div className="subject-area-filter" style={{ position: 'relative', width: '100%' }}>
            <div
                ref={dropdownRef}
                style={{
                    position: 'relative',
                    width: '100%'
                }}
            >
                <button
                    type="button"
                    onClick={() => setIsOpen(!isOpen)}
                    style={{
                        width: '100%',
                        padding: isMobile ? '12px 16px' : '10px 12px',
                        fontSize: '14px',
                        border: '1px solid var(--color-border)',
                        borderRadius: '6px',
                        backgroundColor: 'var(--color-bg-card)',
                        color: 'var(--color-text-primary)',
                        cursor: 'pointer',
                        textAlign: 'left',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        minHeight: isMobile ? '44px' : 'auto'
                    }}
                >
                    <span style={{
                        overflow: 'hidden',
                        textOverflow: 'ellipsis',
                        whiteSpace: 'nowrap',
                        flex: 1,
                        marginRight: '8px'
                    }}>
                        {getDisplayText()}
                    </span>
                    <span style={{
                        fontSize: '12px',
                        color: 'var(--color-text-secondary)',
                        transform: isOpen ? 'rotate(180deg)' : 'rotate(0deg)',
                        transition: 'transform 0.2s ease'
                    }}>
                        ▼
                    </span>
                </button>

                {isOpen && (
                    <div
                        style={{
                            position: 'absolute',
                            top: '100%',
                            left: 0,
                            right: 0,
                            backgroundColor: 'var(--color-bg-card)',
                            border: '1px solid var(--color-border)',
                            borderRadius: '6px',
                            boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)',
                            zIndex: 1000,
                            maxHeight: '300px',
                            overflowY: 'auto',
                            marginTop: '4px'
                        }}
                    >
                        {/* Header with select all/clear all */}
                        <div style={{
                            padding: '8px 12px',
                            borderBottom: '1px solid var(--color-border)',
                            display: 'flex',
                            gap: '8px'
                        }}>
                            <button
                                type="button"
                                onClick={handleSelectAll}
                                style={{
                                    padding: '4px 8px',
                                    fontSize: '12px',
                                    backgroundColor: 'transparent',
                                    color: 'var(--color-accent)',
                                    border: 'none',
                                    cursor: 'pointer',
                                    borderRadius: '4px'
                                }}
                            >
                                Select All
                            </button>
                            <button
                                type="button"
                                onClick={handleClearAll}
                                style={{
                                    padding: '4px 8px',
                                    fontSize: '12px',
                                    backgroundColor: 'transparent',
                                    color: 'var(--color-text-secondary)',
                                    border: 'none',
                                    cursor: 'pointer',
                                    borderRadius: '4px'
                                }}
                            >
                                Clear All
                            </button>
                        </div>

                        {/* Search input */}
                        <div style={{
                            padding: '8px 12px',
                            borderBottom: '1px solid var(--color-border)',
                            position: 'relative'
                        }}>
                            <input
                                ref={searchInputRef}
                                type="text"
                                placeholder="Search subject areas..."
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                style={{
                                    width: '100%',
                                    padding: '6px 8px',
                                    fontSize: '14px',
                                    border: '1px solid var(--color-border)',
                                    borderRadius: '4px',
                                    backgroundColor: 'var(--color-bg-primary)',
                                    color: 'var(--color-text-primary)',
                                    outline: 'none'
                                }}
                                onKeyDown={(e) => {
                                    // Prevent dropdown from closing when typing
                                    e.stopPropagation();
                                }}
                            />
                            {searchQuery && (
                                <button
                                    type="button"
                                    onClick={handleClearSearch}
                                    style={{
                                        position: 'absolute',
                                        right: '20px',
                                        top: '50%',
                                        transform: 'translateY(-50%)',
                                        background: 'none',
                                        border: 'none',
                                        color: 'var(--color-text-secondary)',
                                        cursor: 'pointer',
                                        fontSize: '16px',
                                        padding: '0',
                                        width: '16px',
                                        height: '16px',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center'
                                    }}
                                >
                                    ×
                                </button>
                            )}
                        </div>

                        {/* Subject area options */}
                        <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
                            {filteredAreas.length === 0 ? (
                                <div style={{
                                    padding: '12px',
                                    textAlign: 'center',
                                    color: 'var(--color-text-secondary)',
                                    fontSize: '14px'
                                }}>
                                    No subject areas found matching "{searchQuery}"
                                </div>
                            ) : (
                                filteredAreas.map((area) => {
                                    const isSelected = selectedAreas.includes(area);
                                    return (
                                        <label
                                            key={area}
                                            style={{
                                                display: 'flex',
                                                alignItems: 'center',
                                                padding: '8px 12px',
                                                cursor: 'pointer',
                                                fontSize: '14px',
                                                backgroundColor: isSelected ? 'var(--color-accent-light)' : 'transparent',
                                                color: isSelected ? 'var(--color-accent)' : 'var(--color-text-primary)',
                                                borderBottom: '1px solid var(--color-border-light)'
                                            }}
                                            onMouseEnter={(e) => {
                                                if (!isSelected) {
                                                    e.currentTarget.style.backgroundColor = 'var(--color-bg-hover)';
                                                }
                                            }}
                                            onMouseLeave={(e) => {
                                                if (!isSelected) {
                                                    e.currentTarget.style.backgroundColor = 'transparent';
                                                }
                                            }}
                                        >
                                            <input
                                                type="checkbox"
                                                checked={isSelected}
                                                onChange={() => handleAreaToggle(area)}
                                                style={{
                                                    marginRight: '8px',
                                                    accentColor: 'var(--color-accent)'
                                                }}
                                            />
                                            <span style={{
                                                overflow: 'hidden',
                                                textOverflow: 'ellipsis',
                                                whiteSpace: 'nowrap',
                                                flex: 1
                                            }}>
                                                {area}
                                            </span>
                                        </label>
                                    );
                                })
                            )}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default SubjectAreaFilter;
