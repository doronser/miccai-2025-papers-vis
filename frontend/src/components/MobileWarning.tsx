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

    if (!isMobile || isDismissed) {
        return null;
    }

    return (
        <div className="mobile-warning">
            <div className="mobile-warning-content">
                <div className="mobile-warning-icon">📱</div>
                <div className="mobile-warning-text">
                    <h3>Better Experience on Desktop</h3>
                    <p>
                        This visualization works best on desktop or tablet devices.
                        For the optimal experience, please open this app on a larger screen.
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
