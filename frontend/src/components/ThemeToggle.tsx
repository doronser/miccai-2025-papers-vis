import React from 'react';
import { useTheme, Theme } from '../contexts/ThemeContext';

export const ThemeToggle: React.FC = () => {
    const { theme, setTheme } = useTheme();

    const handleThemeChange = (newTheme: Theme) => {
        setTheme(newTheme);
    };

    return (
        <div className="theme-toggle">
            <button
                onClick={() => handleThemeChange('light')}
                className={theme === 'light' ? 'active' : ''}
                aria-label="Light theme"
                title="Light theme"
            >
                ☀️
            </button>
            <button
                onClick={() => handleThemeChange('dark')}
                className={theme === 'dark' ? 'active' : ''}
                aria-label="Dark theme"
                title="Dark theme"
            >
                🌙
            </button>
            <button
                onClick={() => handleThemeChange('system')}
                className={theme === 'system' ? 'active' : ''}
                aria-label="System theme"
                title="Use system preference"
            >
                💻
            </button>
        </div>
    );
};
