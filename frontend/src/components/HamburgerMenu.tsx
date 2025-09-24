import React from 'react';

interface HamburgerMenuProps {
    isOpen: boolean;
    onToggle: () => void;
    className?: string;
}

export const HamburgerMenu: React.FC<HamburgerMenuProps> = ({
    isOpen,
    onToggle,
    className = ''
}) => {
    return (
        <button
            className={`hamburger-menu ${isOpen ? 'open' : ''} ${className}`}
            onClick={onToggle}
            aria-label={isOpen ? 'Close menu' : 'Open menu'}
            aria-expanded={isOpen}
        >
            <span className="hamburger-line"></span>
            <span className="hamburger-line"></span>
            <span className="hamburger-line"></span>
        </button>
    );
};

export default HamburgerMenu;
