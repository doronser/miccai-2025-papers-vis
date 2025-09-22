import { renderHook } from '@testing-library/react';
import { useMobile } from '../hooks/useMobile';

// Mock window.innerWidth
const mockInnerWidth = (width: number) => {
    Object.defineProperty(window, 'innerWidth', {
        writable: true,
        configurable: true,
        value: width,
    });
};

describe('useMobile', () => {
    beforeEach(() => {
        // Reset window.innerWidth to default
        mockInnerWidth(1024);
    });

    it('should detect desktop breakpoint', () => {
        mockInnerWidth(1200);
        const { result } = renderHook(() => useMobile());

        expect(result.current.isMobile).toBe(false);
        expect(result.current.isTablet).toBe(false);
        expect(result.current.isDesktop).toBe(true);
        expect(result.current.screenWidth).toBe(1200);
    });

    it('should detect tablet breakpoint', () => {
        mockInnerWidth(900);
        const { result } = renderHook(() => useMobile());

        expect(result.current.isMobile).toBe(false);
        expect(result.current.isTablet).toBe(true);
        expect(result.current.isDesktop).toBe(false);
        expect(result.current.screenWidth).toBe(900);
    });

    it('should detect mobile breakpoint', () => {
        mockInnerWidth(600);
        const { result } = renderHook(() => useMobile());

        expect(result.current.isMobile).toBe(true);
        expect(result.current.isTablet).toBe(false);
        expect(result.current.isDesktop).toBe(false);
        expect(result.current.screenWidth).toBe(600);
    });

    it('should update on window resize', () => {
        mockInnerWidth(1200);
        const { result, rerender } = renderHook(() => useMobile());

        expect(result.current.isDesktop).toBe(true);

        // Simulate resize to mobile
        mockInnerWidth(600);
        window.dispatchEvent(new Event('resize'));
        rerender();

        expect(result.current.isMobile).toBe(true);
        expect(result.current.isDesktop).toBe(false);
    });
});
