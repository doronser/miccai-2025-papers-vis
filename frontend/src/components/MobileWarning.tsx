import React, { useState, useEffect } from 'react';

const MobileWarning: React.FC = () => {
    const [isMobile, setIsMobile] = useState(false);
    const [isDismissed, setIsDismissed] = useState(false);

    useEffect(() => {
        const checkMobile = () => {
            const userAgent = navigator.userAgent.toLowerCase();
            const mobileKeywords = ['mobile', 'android', 'iphone', 'ipad', 'tablet'];
            const isMobileDevice = mobileKeywords.some(keyword => userAgent.includes(keyword));

            // Also check screen size
            const isSmallScreen = window.innerWidth < 768;

            setIsMobile(isMobileDevice || isSmallScreen);
        };

        checkMobile();
        window.addEventListener('resize', checkMobile);

        return () => window.removeEventListener('resize', checkMobile);
    }, []);

    // Don't show warning if dismissed or if user is on a larger screen
    if (!isMobile || isDismissed || window.innerWidth >= 768) {
        return null;
    }

    return (
        <div className="mobile-warning">
            <div className="mobile-warning-content">
                <div className="mobile-warning-icon">📱</div>
                <div className="mobile-warning-text">
                    <h3>Mobile Experience Available</h3>
                    <p>
                        This visualization now supports mobile devices! Use the menu button to search and explore papers.
                    </p>
                </div>
                <button
                    className="mobile-warning-close"
                    onClick={() => setIsDismissed(true)}
                    aria-label="Dismiss warning"
                >
                    ×
                </button>
            </div>
        </div>
    );
};

export default MobileWarning;
