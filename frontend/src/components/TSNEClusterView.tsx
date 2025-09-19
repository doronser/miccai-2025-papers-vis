import React, { useEffect, useRef, useState, useCallback } from 'react';
import * as d3 from 'd3';
import { apiService } from '../services/api';

interface TSNECoordinate {
    paper_id: string;
    tsne_x: number;
    tsne_y: number;
    subject_areas: string[];
    title: string;
    authors: string[];
}

interface TSNEClusterViewProps {
    width: number;
    height: number;
    onNodeClick?: (paperId: string) => void;
    selectedPaperId?: string | null;
    searchQuery?: string;
}

const TSNEClusterView: React.FC<TSNEClusterViewProps> = ({
    width,
    height,
    onNodeClick,
    selectedPaperId,
    searchQuery = ''
}) => {
    const svgRef = useRef<SVGSVGElement>(null);
    const [coordinates, setCoordinates] = useState<TSNECoordinate[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [zoom, setZoom] = useState<d3.ZoomTransform>(d3.zoomIdentity);
    const [hoveredPaperId, setHoveredPaperId] = useState<string | null>(null);
    const [initialTransform, setInitialTransform] = useState<d3.ZoomTransform | null>(null);
    const [themeChangeKey, setThemeChangeKey] = useState<number>(0);

    // Listen for theme changes
    useEffect(() => {
        const observer = new MutationObserver(() => {
            setThemeChangeKey(prev => prev + 1);
        });

        observer.observe(document.documentElement, {
            attributes: true,
            attributeFilter: ['data-theme']
        });

        return () => observer.disconnect();
    }, []);

    // Load t-SNE coordinates
    useEffect(() => {
        const loadCoordinates = async () => {
            try {
                setLoading(true);
                setError(null);
                const response = await apiService.getTSNECoordinates();
                setCoordinates(response.coordinates);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'Failed to load coordinates');
            } finally {
                setLoading(false);
            }
        };

        loadCoordinates();
    }, []);

    // Filter coordinates based on search query only
    const filteredCoordinates = React.useMemo(() => {
        return coordinates.filter(coord => {
            // Filter by search query
            if (searchQuery) {
                const query = searchQuery.toLowerCase();
                const matchesSearch =
                    coord.title.toLowerCase().includes(query) ||
                    coord.authors.some(author => author.toLowerCase().includes(query)) ||
                    coord.subject_areas.some(area => area.toLowerCase().includes(query));
                if (!matchesSearch) return false;
            }
            return true;
        });
    }, [coordinates, searchQuery]);

    // Render the visualization
    useEffect(() => {
        if (!svgRef.current || loading || error) return;

        const svg = d3.select(svgRef.current);
        svg.selectAll('*').remove();

        // Add subtle background
        svg.append('rect')
            .attr('width', width)
            .attr('height', height)
            .attr('fill', 'var(--color-graph-bg)')
            .attr('rx', 8);

        if (filteredCoordinates.length === 0) {
            svg.append('text')
                .attr('x', width / 2)
                .attr('y', height / 2)
                .attr('text-anchor', 'middle')
                .attr('fill', '#666')
                .text('No papers match the current filters');
            return;
        }

        // Create scales
        const xExtent = d3.extent(filteredCoordinates, d => d.tsne_x) as [number, number];
        const yExtent = d3.extent(filteredCoordinates, d => d.tsne_y) as [number, number];

        const xScale = d3.scaleLinear()
            .domain(xExtent)
            .range([50, width - 50]);

        const yScale = d3.scaleLinear()
            .domain(yExtent)
            .range([height - 50, 50]);

        // Create zoom behavior
        const zoomBehavior = d3.zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.1, 10])
            .on('zoom', (event) => {
                setZoom(event.transform);
                svg.selectAll('.node-group')
                    .attr('transform', (d: any) => {
                        const x = xScale(d.tsne_x);
                        const y = yScale(d.tsne_y);
                        return `translate(${event.transform.applyX(x)}, ${event.transform.applyY(y)})`;
                    });
            });

        svg.call(zoomBehavior);

        // Center the view at 100% zoom
        const centerX = (xExtent[0] + xExtent[1]) / 2;
        const centerY = (yExtent[0] + yExtent[1]) / 2;
        const translateX = width / 2 - xScale(centerX);
        const translateY = height / 2 - yScale(centerY);

        const initialTransform = d3.zoomIdentity
            .translate(translateX, translateY)
            .scale(1);

        svg.call(zoomBehavior.transform, initialTransform);
        setZoom(initialTransform);
        setInitialTransform(initialTransform);

        // Create nodes
        const nodes = svg.selectAll('.node-group')
            .data(filteredCoordinates)
            .enter()
            .append('g')
            .attr('class', 'node-group')
            .attr('transform', d => `translate(${xScale(d.tsne_x)}, ${yScale(d.tsne_y)})`)
            .style('cursor', 'pointer');

        // Add circles
        nodes.append('circle')
            .attr('r', 4)
            .attr('fill', 'var(--color-node-default)')
            .attr('stroke', 'var(--color-node-border)')
            .attr('stroke-width', 1.5)
            .attr('opacity', 0.8);

        // Add tooltips with paper title (first 100 characters)
        nodes.append('title')
            .text(d => d.title.length > 100 ? d.title.substring(0, 100) + '...' : d.title);

        // Add click handlers
        nodes.on('click', (_, d) => {
            onNodeClick?.(d.paper_id);
        });

        // Add hover handlers (visual only, no paper info update)
        nodes.on('mouseenter', (_, d) => {
            setHoveredPaperId(d.paper_id);
        });

        nodes.on('mouseleave', () => {
            setHoveredPaperId(null);
        });

    }, [filteredCoordinates, width, height, loading, error]);

    // Update visual states - runs when selection/hover changes
    useEffect(() => {
        if (!svgRef.current || loading || error) return;

        const svg = d3.select(svgRef.current);

        svg.selectAll('.node-group circle')
            .attr('r', (d: any) => {
                if (d.paper_id === selectedPaperId) return 8;
                if (d.paper_id === hoveredPaperId) return 6;
                return 4;
            })
            .attr('fill', (d: any) => {
                if (d.paper_id === selectedPaperId) return 'var(--color-node-selected)';
                if (d.paper_id === hoveredPaperId) return 'var(--color-node-hover)';
                return 'var(--color-node-default)';
            })
            .attr('stroke', (d: any) => {
                if (d.paper_id === selectedPaperId) return 'var(--color-node-selected)';
                if (d.paper_id === hoveredPaperId) return 'var(--color-node-hover)';
                return 'var(--color-node-border)';
            })
            .attr('stroke-width', (d: any) => {
                if (d.paper_id === selectedPaperId || d.paper_id === hoveredPaperId) return 2;
                return 1;
            });

    }, [selectedPaperId, hoveredPaperId, themeChangeKey]);

    // Reset zoom function
    const resetZoom = useCallback(() => {
        if (svgRef.current && initialTransform) {
            const svg = d3.select(svgRef.current);
            svg.transition().duration(750).call(
                d3.zoom<SVGSVGElement, unknown>().transform,
                initialTransform
            );
            setZoom(initialTransform);
        }
    }, [initialTransform]);

    if (loading) {
        return (
            <div className="flex items-center justify-center" style={{ width, height }}>
                <div className="text-gray-600">Loading t-SNE visualization...</div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="flex items-center justify-center" style={{ width, height }}>
                <div className="text-red-600">Error: {error}</div>
            </div>
        );
    }

    return (
        <div className="relative">
            <svg
                ref={svgRef}
                width={width}
                height={height}
                style={{
                    borderRadius: 'var(--radius-lg)',
                    boxShadow: 'var(--shadow-md)',
                    border: '1px solid var(--color-border)'
                }}
            />

            {/* Reset Zoom Button */}
            <button
                onClick={resetZoom}
                className="btn"
                style={{
                    position: 'absolute',
                    top: '10px',
                    left: '10px',
                    fontSize: '14px'
                }}
                title="Reset zoom to fit all papers"
            >
                Reset View
            </button>


            {/* Info panel */}
            <div className="card" style={{
                position: 'absolute',
                bottom: '10px',
                left: '10px',
                padding: 'var(--spacing-md)',
                fontSize: '12px'
            }}>
                <div style={{ color: 'var(--color-text-secondary)' }}>
                    <div>Papers: {filteredCoordinates.length}</div>
                    <div>Zoom: {(zoom.k * 100).toFixed(0)}%</div>
                    {selectedPaperId && (
                        <div style={{ color: 'var(--color-accent)', fontWeight: '500', marginTop: '4px' }}>
                            Selected: {coordinates.find(c => c.paper_id === selectedPaperId)?.title.substring(0, 30)}...
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default TSNEClusterView;
