import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeContextType {
    theme: Theme;
    setTheme: (theme: Theme) => void;
    resolvedTheme: 'light' | 'dark';
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

interface ThemeProviderProps {
    children: ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
    const [theme, setTheme] = useState<Theme>('system');
    const [resolvedTheme, setResolvedTheme] = useState<'light' | 'dark'>('light');

    // Load saved preference from localStorage on mount
    useEffect(() => {
        const savedTheme = localStorage.getItem('theme') as Theme | null;
        if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
            setTheme(savedTheme);
        }
    }, []);

    // Handle system preference changes
    useEffect(() => {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

        const handleChange = () => {
            if (theme === 'system') {
                setResolvedTheme(mediaQuery.matches ? 'dark' : 'light');
            }
        };

        if (theme === 'system') {
            setResolvedTheme(mediaQuery.matches ? 'dark' : 'light');
            mediaQuery.addEventListener('change', handleChange);
        } else {
            setResolvedTheme(theme);
        }

        return () => mediaQuery.removeEventListener('change', handleChange);
    }, [theme]);

    // Apply theme to document and save to localStorage
    useEffect(() => {
        document.documentElement.setAttribute('data-theme', resolvedTheme);
        localStorage.setItem('theme', theme);
    }, [theme, resolvedTheme]);

    const value: ThemeContextType = {
        theme,
        setTheme,
        resolvedTheme
    };

    return (
        <ThemeContext.Provider value={value}>
            {children}
        </ThemeContext.Provider>
    );
};

export const useTheme = (): ThemeContextType => {
    const context = useContext(ThemeContext);
    if (context === undefined) {
        throw new Error('useTheme must be used within a ThemeProvider');
    }
    return context;
};
