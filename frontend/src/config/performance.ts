// Performance configuration for MICCAI Papers Visualization

export interface PerformanceConfig {
  // Graph visualization limits
  maxNodes: number;
  maxEdges: number;
  similarityThreshold: number;

  // Interaction timing
  debounceDelay: number;
  animationDuration: number;

  // Memory management
  cacheSize: number;
  preloadSimilarPapers: number;

  // Progressive loading
  initialLoadSize: number;
  loadBatchSize: number;
}

// Default configuration optimized for 1000+ papers
export const DEFAULT_CONFIG: PerformanceConfig = {
  maxNodes: 500,                // Limit nodes for smooth rendering
  maxEdges: 1000,              // Limit edges for performance
  similarityThreshold: 0.75,   // Higher threshold = fewer edges

  debounceDelay: 300,          // Search input debounce (ms)
  animationDuration: 1000,     // D3 transition duration (ms)

  cacheSize: 100,              // Max cached papers in memory
  preloadSimilarPapers: 5,     // Similar papers to preload

  initialLoadSize: 200,        // Papers to load initially
  loadBatchSize: 50,           // Additional papers per batch
};

// Performance presets for different use cases
export const PERFORMANCE_PRESETS = {
  // For development/testing with small datasets
  DEVELOPMENT: {
    ...DEFAULT_CONFIG,
    maxNodes: 100,
    maxEdges: 200,
    initialLoadSize: 50,
  } as PerformanceConfig,

  // Balanced performance for most users
  BALANCED: DEFAULT_CONFIG,

  // Maximum quality for powerful machines
  HIGH_QUALITY: {
    ...DEFAULT_CONFIG,
    maxNodes: 1000,
    maxEdges: 2000,
    similarityThreshold: 0.7,
    initialLoadSize: 500,
  } as PerformanceConfig,

  // Optimized for mobile/slower devices
  MOBILE: {
    ...DEFAULT_CONFIG,
    maxNodes: 200,
    maxEdges: 300,
    similarityThreshold: 0.8,
    animationDuration: 500,
    initialLoadSize: 100,
    loadBatchSize: 25,
  } as PerformanceConfig,
};

// Auto-detect optimal configuration based on device capabilities
export function detectOptimalConfig(): PerformanceConfig {
  // Check if running on mobile
  const isMobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

  // Estimate device performance based on hardware concurrency and memory
  const cores = navigator.hardwareConcurrency || 4;
  const memory = (navigator as any).deviceMemory || 4; // GB

  // Performance score (higher = more capable device)
  const performanceScore = cores * memory;

  if (isMobile) {
    return PERFORMANCE_PRESETS.MOBILE;
  } else if (performanceScore >= 16) { // 4+ cores, 4+ GB RAM
    return PERFORMANCE_PRESETS.HIGH_QUALITY;
  } else if (performanceScore >= 8) {  // 2+ cores, 4+ GB RAM or 4+ cores, 2+ GB RAM
    return PERFORMANCE_PRESETS.BALANCED;
  } else {
    return PERFORMANCE_PRESETS.MOBILE;
  }
}

// Performance monitoring utilities
export class PerformanceMonitor {
  private static metrics: Record<string, number[]> = {};

  static startTimer(label: string): () => void {
    const start = performance.now();
    return () => {
      const duration = performance.now() - start;
      this.recordMetric(label, duration);
      return duration;
    };
  }

  static recordMetric(label: string, value: number): void {
    if (!this.metrics[label]) {
      this.metrics[label] = [];
    }

    this.metrics[label].push(value);

    // Keep only last 100 measurements
    if (this.metrics[label].length > 100) {
      this.metrics[label].shift();
    }
  }

  static getMetrics(): Record<string, { avg: number; min: number; max: number; count: number }> {
    const summary: Record<string, { avg: number; min: number; max: number; count: number }> = {};

    for (const [label, values] of Object.entries(this.metrics)) {
      const avg = values.reduce((a, b) => a + b, 0) / values.length;
      const min = Math.min(...values);
      const max = Math.max(...values);

      summary[label] = { avg, min, max, count: values.length };
    }

    return summary;
  }

  static logPerformanceSummary(): void {
    const metrics = this.getMetrics();
    console.group('🔍 Performance Metrics Summary');

    for (const [label, stats] of Object.entries(metrics)) {
      console.log(`${label}: avg=${stats.avg.toFixed(2)}ms, min=${stats.min.toFixed(2)}ms, max=${stats.max.toFixed(2)}ms (n=${stats.count})`);
    }

    console.groupEnd();
  }
}

// React hook for performance configuration
import { useState } from 'react';

export function usePerformanceConfig(): [PerformanceConfig, (config: Partial<PerformanceConfig>) => void] {
  const [config, setConfig] = useState<PerformanceConfig>(() => {
    // Try to load from localStorage first
    const saved = localStorage.getItem('miccai-performance-config');
    if (saved) {
      try {
        return { ...DEFAULT_CONFIG, ...JSON.parse(saved) };
      } catch {
        // Fall back to auto-detection if parsing fails
      }
    }

    return detectOptimalConfig();
  });

  const updateConfig = (newConfig: Partial<PerformanceConfig>) => {
    const updatedConfig = { ...config, ...newConfig };
    setConfig(updatedConfig);
    localStorage.setItem('miccai-performance-config', JSON.stringify(updatedConfig));
  };

  return [config, updateConfig];
}
