import { useState, useEffect } from 'react';

export interface MobileBreakpoints {
    isMobile: boolean;
    isTablet: boolean;
    isDesktop: boolean;
    screenWidth: number;
}

export const useMobile = (): MobileBreakpoints => {
    const [breakpoints, setBreakpoints] = useState<MobileBreakpoints>({
        isMobile: false,
        isTablet: false,
        isDesktop: true,
        screenWidth: window.innerWidth,
    });

    useEffect(() => {
        const updateBreakpoints = () => {
            const width = window.innerWidth;
            setBreakpoints({
                isMobile: width < 768,
                isTablet: width >= 768 && width < 1024,
                isDesktop: width >= 1024,
                screenWidth: width,
            });
        };

        updateBreakpoints();
        window.addEventListener('resize', updateBreakpoints);

        return () => window.removeEventListener('resize', updateBreakpoints);
    }, []);

    return breakpoints;
};
