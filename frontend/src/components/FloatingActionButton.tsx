import React from 'react';

interface FloatingActionButtonProps {
    onClick: () => void;
    icon: React.ReactNode;
    label: string;
    className?: string;
    disabled?: boolean;
}

export const FloatingActionButton: React.FC<FloatingActionButtonProps> = ({
    onClick,
    icon,
    label,
    className = '',
    disabled = false
}) => {
    return (
        <button
            className={`floating-action-button ${className}`}
            onClick={onClick}
            disabled={disabled}
            aria-label={label}
            title={label}
        >
            {icon}
        </button>
    );
};

export default FloatingActionButton;
