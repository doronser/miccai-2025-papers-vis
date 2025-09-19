import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { GraphData, GraphNode } from '../types/api';
import { useTheme } from '../contexts/ThemeContext';

interface ClusterViewProps {
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
    vx?: number;
    vy?: number;
}

const ClusterView: React.FC<ClusterViewProps> = ({
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

        // Use t-SNE coordinates if available, otherwise use force simulation
        let simulation: d3.Simulation<D3Node, undefined> | null = null;

        if (data.nodes.some(node => node.x !== undefined && node.y !== undefined)) {
            // Use t-SNE coordinates - no simulation needed
            console.log('Using t-SNE coordinates for cluster view');
        } else {
            // Create simulation for cluster positioning
            simulation = d3.forceSimulation<D3Node>(data.nodes)
                .force('charge', d3.forceManyBody().strength(-300)) // Stronger repulsion for better separation
                .force('center', d3.forceCenter(width / 2, height / 2))
                .force('collision', d3.forceCollide().radius(20)) // Larger collision radius to prevent overlap
                .force('cluster', clusterForce()) // Custom force for clustering
                .alphaDecay(0.01)
                .alphaMin(0.001);
        }

        // Custom force to group nodes by cluster
        function clusterForce() {
            const strength = 0.1;
            return function (alpha: number) {
                data.nodes.forEach((node) => {
                    if (node.cluster === undefined) return;

                    // Find other nodes in the same cluster
                    const clusterNodes = data.nodes.filter(n => n.cluster === node.cluster);
                    const clusterCenter = { x: 0, y: 0 };

                    clusterNodes.forEach(n => {
                        clusterCenter.x += (n.x || 0);
                        clusterCenter.y += (n.y || 0);
                    });

                    clusterCenter.x /= clusterNodes.length;
                    clusterCenter.y /= clusterNodes.length;

                    // Apply force towards cluster center
                    const dx = clusterCenter.x - (node.x || 0);
                    const dy = clusterCenter.y - (node.y || 0);

                    (node as D3Node).vx = ((node as D3Node).vx || 0) + dx * strength * alpha;
                    (node as D3Node).vy = ((node as D3Node).vy || 0) + dy * strength * alpha;
                });
            };
        }

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

        // Add circles for nodes with reasonable size
        node.append('circle')
            .attr('r', 8) // Smaller nodes for better visibility
            .attr('fill', (d: GraphNode) => d.cluster !== undefined ? colorScale(String(d.cluster)) : 'var(--color-node-default)')
            .attr('stroke', 'var(--color-node-border)')
            .attr('stroke-width', 1.5);

        // Add labels (only show on hover)
        node.append('text')
            .attr('dx', 12) // Adjusted for smaller nodes
            .attr('dy', '.35em')
            .attr('font-size', '11px') // Smaller font
            .attr('fill', 'var(--color-text-primary)')
            .style('pointer-events', 'none')
            .style('opacity', 0) // Hidden by default
            .text((d: GraphNode) => d.title.length > 30 ? d.title.substring(0, 30) + '...' : d.title);

        // Add interactions
        node
            .on('click', (_, d) => {
                onNodeClick?.(d);
            })
            .on('mouseenter', (event, d) => {
                onNodeHover?.(d);
                // Highlight this node
                d3.select(event.currentTarget).select('circle')
                    .attr('r', 12)
                    .attr('stroke-width', 2.5);

                // Show label
                d3.select(event.currentTarget).select('text').style('opacity', 1);
            })
            .on('mouseleave', () => {
                onNodeHover?.(null);
                // Reset all nodes
                node.selectAll('circle')
                    .attr('r', 8)
                    .attr('stroke-width', 1.5);
                node.selectAll('text').style('opacity', 0); // Hide all labels
            });

        // Update positions on simulation tick or use t-SNE coordinates
        if (simulation) {
            simulation.on('tick', () => {
                node.attr('transform', (d: D3Node) => `translate(${d.x},${d.y})`);
            });
        } else {
            // Scale and center similarity coordinates to fit the visualization area with better spacing
            const nodes = data.nodes.filter(n => n.x !== undefined && n.y !== undefined);
            if (nodes.length > 0) {
                // Calculate bounds of similarity coordinates
                const xExtent = d3.extent(nodes, d => d.x!) as [number, number];
                const yExtent = d3.extent(nodes, d => d.y!) as [number, number];

                // Calculate scaling factors to use most of the available space with more aggressive scaling
                const padding = 50; // Reduced padding for more space
                const scaleX = (width - padding) / (xExtent[1] - xExtent[0]);
                const scaleY = (height - padding) / (yExtent[1] - yExtent[0]);
                const scale = Math.min(scaleX, scaleY) * 0.95; // Use 95% of available space for better spread

                // Calculate center offset
                const centerX = width / 2;
                const centerY = height / 2;
                const offsetX = centerX - (xExtent[0] + xExtent[1]) / 2 * scale;
                const offsetY = centerY - (yExtent[0] + yExtent[1]) / 2 * scale;

                // Apply scaling and centering to all nodes with additional spread factor
                const spreadFactor = 1.5; // Additional spread to reduce clumping
                node.attr('transform', (d: D3Node) => {
                    const scaledX = (d.x || 0) * scale * spreadFactor + offsetX;
                    const scaledY = (d.y || 0) * scale * spreadFactor + offsetY;
                    return `translate(${scaledX},${scaledY})`;
                });
            } else {
                // Fallback if no coordinates
                node.attr('transform', (d: D3Node) => `translate(${d.x || 0},${d.y || 0})`);
            }
        }

        // Drag functions
        function dragstarted(event: any, d: D3Node) {
            if (simulation && !event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }

        function dragged(event: any, d: D3Node) {
            d.fx = event.x;
            d.fy = event.y;
        }

        function dragended(event: any, d: D3Node) {
            if (simulation && !event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }

        return () => {
            if (simulation) {
                simulation.stop();
            }
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

export default ClusterView;
