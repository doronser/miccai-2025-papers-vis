import React, { useState, useEffect, useRef } from 'react';
import ClusterView from './ClusterView';
import SimilarityNetworkView from './SimilarityNetworkView';
import GraphLegend from './GraphLegend';
import { GraphData, GraphNode, ViewMode } from '../types/api';

interface GraphVisualizationProps {
  data: GraphData;
  width?: number;
  height?: number;
  onNodeClick?: (node: GraphNode) => void;
  onNodeHover?: (node: GraphNode | null) => void;
  initialViewMode?: ViewMode;
  selectedPaperId?: string | null;
  onViewModeChange?: (mode: ViewMode) => void;
}

const GraphVisualization: React.FC<GraphVisualizationProps> = ({
  data,
  width = 1200,
  height = 800,
  onNodeClick,
  onNodeHover,
  initialViewMode = 'clusters',
  selectedPaperId,
  onViewModeChange
}) => {
  const [viewMode, setViewMode] = useState<ViewMode>(initialViewMode);
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width, height });

  // Auto-fit to screen
  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setDimensions({
          width: rect.width,
          height: rect.height
        });
      }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  const handleViewModeChange = (mode: ViewMode) => {
    setViewMode(mode);
    onViewModeChange?.(mode);
  };

  return (
    <div style={{ width: '100%', height: '100%' }}>
      {/* Graph Container */}
      <div
        ref={containerRef}
        className="graph-container"
        style={{
          position: 'relative',
          width: '100%',
          height: 'calc(100% - 80px)', // Much less space reserved for legend
          minHeight: '600px'
        }}
      >
        {/* Render appropriate view */}
        {viewMode === 'clusters' ? (
          <ClusterView
            data={data}
            width={dimensions.width}
            height={dimensions.height - 80} // Much less space for legend
            onNodeClick={onNodeClick}
            onNodeHover={onNodeHover}
          />
        ) : (
          <SimilarityNetworkView
            data={data}
            width={dimensions.width}
            height={dimensions.height - 80} // Much less space for legend
            onNodeClick={onNodeClick}
            onNodeHover={onNodeHover}
          />
        )}
      </div>

      {/* Legend positioned below the graph - much smaller */}
      <div style={{
        width: '100%',
        height: '80px', // Much smaller legend height
        padding: '5px 0',
        overflowY: 'auto'
      }}>
        <GraphLegend
          data={data}
          viewMode={viewMode}
          selectedPaperId={selectedPaperId}
        />
      </div>
    </div>
  );
};

export default GraphVisualization;
