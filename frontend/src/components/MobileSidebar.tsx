import React from 'react';

interface MobileSidebarProps {
    isOpen: boolean;
    onClose: () => void;
    children: React.ReactNode;
    className?: string;
}

export const MobileSidebar: React.FC<MobileSidebarProps> = ({
    isOpen,
    onClose,
    children,
    className = ''
}) => {
    return (
        <>
            {/* Backdrop */}
            {isOpen && (
                <div
                    className="mobile-sidebar-backdrop"
                    onClick={onClose}
                    aria-hidden="true"
                />
            )}

            {/* Sidebar */}
            <div className={`mobile-sidebar ${isOpen ? 'open' : ''} ${className}`}>
                <div className="mobile-sidebar-header">
                    <h3>Navigation</h3>
                    <button
                        className="mobile-sidebar-close"
                        onClick={onClose}
                        aria-label="Close sidebar"
                    >
                        ×
                    </button>
                </div>
                <div className="mobile-sidebar-content">
                    {children}
                </div>
            </div>
        </>
    );
};

export default MobileSidebar;
