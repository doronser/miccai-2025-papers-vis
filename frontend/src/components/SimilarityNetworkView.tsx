import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { GraphData, GraphNode, GraphEdge } from '../types/api';
import { useTheme } from '../contexts/ThemeContext';

interface SimilarityNetworkViewProps {
    data: GraphData;
    width: number;
    height: number;
    onNodeClick?: (node: GraphNode) => void;
    onNodeHover?: (node: GraphNode | null) => void;
}

interface D3Node extends GraphNode {
    x?: number;
    y?: number;
    fx?: number | null;
    fy?: number | null;
}

interface D3Edge {
    source: D3Node;
    target: D3Node;
    similarity: number;
}

const SimilarityNetworkView: React.FC<SimilarityNetworkViewProps> = ({
    data,
    width,
    height,
    onNodeClick,
    onNodeHover
}) => {
    const svgRef = useRef<SVGSVGElement>(null);
    const { resolvedTheme } = useTheme();

    useEffect(() => {
        if (!data || !data.nodes.length) return;

        const svg = d3.select(svgRef.current);
        svg.selectAll('*').remove(); // Clear previous render

        // Create zoom behavior
        const zoom = d3.zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.1, 4])
            .on('zoom', (event) => {
                container.attr('transform', event.transform);
            });

        svg.call(zoom as any);

        const container = svg.append('g');

        // Color scale for clusters using theme variables
        const colorScale = d3.scaleOrdinal([
            'var(--color-cluster-0)',
            'var(--color-cluster-1)',
            'var(--color-cluster-2)',
            'var(--color-cluster-3)',
            'var(--color-cluster-4)',
            'var(--color-cluster-5)',
            'var(--color-cluster-6)',
            'var(--color-cluster-7)',
            'var(--color-cluster-8)',
            'var(--color-cluster-9)'
        ]);

        // Create arrow markers for directed edges
        svg.append('defs').selectAll('marker')
            .data(['end'])
            .enter().append('marker')
            .attr('id', String)
            .attr('viewBox', '0 -5 10 10')
            .attr('refX', 25)
            .attr('refY', 0)
            .attr('markerWidth', 6)
            .attr('markerHeight', 6)
            .attr('orient', 'auto')
            .append('path')
            .attr('d', 'M0,-5L10,0L0,5')
            .attr('fill', 'var(--color-edge)');

        // Convert edges to D3Edge format
        const d3Edges: D3Edge[] = data.edges.map(edge => ({
            source: data.nodes.find(n => n.id === edge.source) as D3Node,
            target: data.nodes.find(n => n.id === edge.target) as D3Node,
            similarity: edge.similarity
        }));

        // Create simulation
        const simulation = d3.forceSimulation<D3Node, D3Edge>(data.nodes)
            .force('link', d3.forceLink<D3Node, D3Edge>(d3Edges)
                .id((d: any) => d.id)
                .distance(80)
                .strength(0.05))
            .force('charge', d3.forceManyBody().strength(-80))
            .force('center', d3.forceCenter(width / 2, height / 2))
            .force('collision', d3.forceCollide().radius(10))
            .alphaDecay(0.01)
            .alphaMin(0.001);

        // Create edges with similarity-based styling
        const link = container.append('g')
            .attr('class', 'links')
            .selectAll('line')
            .data(data.edges)
            .enter().append('line')
            .attr('stroke', (d: GraphEdge) => {
                // Color based on similarity: higher similarity = warmer color
                const similarity = d.similarity;
                if (similarity > 0.8) return '#ff6b6b'; // High similarity - red
                if (similarity > 0.6) return '#4ecdc4'; // Medium-high similarity - teal
                if (similarity > 0.4) return '#45b7d1'; // Medium similarity - blue
                return '#96ceb4'; // Low similarity - green
            })
            .attr('stroke-opacity', (d: GraphEdge) => Math.max(0.3, d.similarity))
            .attr('stroke-width', (d: GraphEdge) => {
                // Edge width based on similarity (distance = 1 - similarity)
                const distance = 1 - d.similarity;
                return Math.max(1, Math.min(8, distance * 10));
            })
            .attr('stroke-dasharray', (d: GraphEdge) => {
                // Dashed lines for lower similarity
                return d.similarity < 0.5 ? '5,5' : 'none';
            })
            .style('opacity', 0.8); // Set initial opacity to make edges visible

        // Create nodes
        const node = container.append('g')
            .attr('class', 'nodes')
            .selectAll('g')
            .data(data.nodes)
            .enter().append('g')
            .attr('class', 'node-group')
            .call(d3.drag<SVGGElement, D3Node>()
                .on('start', dragstarted)
                .on('drag', dragged)
                .on('end', dragended));

        // Add circles for nodes with special styling for central node
        node.append('circle')
            .attr('r', (d: GraphNode) => d.cluster === 0 ? 12 : 6) // Smaller, more reasonable sizes
            .attr('fill', (d: GraphNode) => {
                if (d.cluster === 0) return 'var(--color-node-selected)'; // Central node gets special color
                return d.cluster !== undefined ? colorScale(String(d.cluster)) : 'var(--color-node-default)';
            })
            .attr('stroke', (d: GraphNode) => d.cluster === 0 ? 'var(--color-node-border)' : 'var(--color-node-border)')
            .attr('stroke-width', (d: GraphNode) => d.cluster === 0 ? 2.5 : 1.5); // Central node has thicker border

        // Add labels (only show on hover, but always show for central node)
        node.append('text')
            .attr('dx', (d: GraphNode) => d.cluster === 0 ? 16 : 10) // Adjusted for smaller nodes
            .attr('dy', '.35em')
            .attr('font-size', (d: GraphNode) => d.cluster === 0 ? '12px' : '10px') // Smaller fonts
            .attr('fill', 'var(--color-text-primary)')
            .style('pointer-events', 'none')
            .style('opacity', (d: GraphNode) => d.cluster === 0 ? 1 : 0) // Always show central node label
            .text((d: GraphNode) => d.title.length > 40 ? d.title.substring(0, 40) + '...' : d.title);

        // Add interactions
        node
            .on('click', (_, d) => {
                onNodeClick?.(d);
            })
            .on('mouseenter', (event, d) => {
                onNodeHover?.(d);
                // Show label for this node
                d3.select(event.currentTarget).select('text').style('opacity', 1);

                // Highlight connected nodes
                const connectedNodes = new Set([d.id]);
                data.edges.forEach(edge => {
                    if (edge.source === d.id) connectedNodes.add(edge.target);
                    if (edge.target === d.id) connectedNodes.add(edge.source);
                });

                node.style('opacity', (n: GraphNode) => connectedNodes.has(n.id) ? 1 : 0.3);
                link.style('opacity', (e: GraphEdge) =>
                    e.source === d.id || e.target === d.id ? 1 : 0.1);
            })
            .on('mouseleave', () => {
                onNodeHover?.(null);
                // Hide all labels again
                node.selectAll('text').style('opacity', 0);
                node.style('opacity', 1);
                link.style('opacity', 0.8); // Make edges more visible by default
            });

        // Update positions on simulation tick
        simulation.on('tick', () => {
            link
                .attr('x1', (d: any) => d.source.x)
                .attr('y1', (d: any) => d.source.y)
                .attr('x2', (d: any) => d.target.x)
                .attr('y2', (d: any) => d.target.y);

            node.attr('transform', (d: D3Node) => `translate(${d.x},${d.y})`);
        });

        // Drag functions
        function dragstarted(event: any, d: D3Node) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }

        function dragged(event: any, d: D3Node) {
            d.fx = event.x;
            d.fy = event.y;
        }

        function dragended(event: any, d: D3Node) {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }

        return () => {
            simulation.stop();
        };
    }, [data, width, height, onNodeClick, onNodeHover, resolvedTheme]);

    return (
        <svg
            ref={svgRef}
            width={width}
            height={height}
            style={{
                border: '1px solid var(--color-border)',
                background: 'var(--color-bg-secondary)',
                borderRadius: '8px'
            }}
        />
    );
};

export default SimilarityNetworkView;
