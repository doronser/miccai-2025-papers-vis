import React, { useState } from 'react';
import { GraphData, ViewMode } from '../types/api';

interface GraphLegendProps {
    data: GraphData;
    viewMode: ViewMode;
    selectedPaperId?: string | null;
}

const GraphLegend: React.FC<GraphLegendProps> = ({
    data,
    viewMode,
    selectedPaperId
}) => {
    const [isCollapsed, setIsCollapsed] = useState(false);
    // Get cluster information
    const clusterInfo = React.useMemo(() => {
        if (!data.nodes.length) return {};

        const clusters: Record<number, { count: number; papers: string[] }> = {};

        data.nodes.forEach(node => {
            if (node.cluster !== undefined) {
                if (!clusters[node.cluster]) {
                    clusters[node.cluster] = { count: 0, papers: [] };
                }
                clusters[node.cluster].count++;
                clusters[node.cluster].papers.push(node.title);
            }
        });

        return clusters;
    }, [data.nodes]);

    // Get subject area information
    const subjectAreaInfo = React.useMemo(() => {
        if (!data.nodes.length) return {};

        const areas: Record<string, number> = {};

        data.nodes.forEach(node => {
            node.subject_areas.forEach(area => {
                areas[area] = (areas[area] || 0) + 1;
            });
        });

        return areas;
    }, [data.nodes]);

    const clusterColors = [
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
    ];

    return (
        <div style={{
            background: 'var(--color-bg-card)',
            padding: '8px', // Reduced padding
            borderRadius: '6px', // Smaller border radius
            fontSize: '10px', // Smaller font
            color: 'var(--color-text-primary)',
            border: '1px solid var(--color-border)',
            boxShadow: 'var(--shadow-sm)',
            maxWidth: '100%',
            maxHeight: isCollapsed ? 'auto' : '60px', // Much smaller max height
            overflowY: isCollapsed ? 'visible' : 'auto'
        }}>
            <div style={{
                marginBottom: isCollapsed ? '0' : '6px', // Reduced margin
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center'
            }}>
                <strong style={{ fontSize: '11px' }}>Legend</strong> {/* Smaller font */}
                <button
                    onClick={() => setIsCollapsed(!isCollapsed)}
                    style={{
                        background: 'none',
                        border: 'none',
                        fontSize: '12px', // Smaller button
                        cursor: 'pointer',
                        padding: '2px',
                        color: 'var(--color-text-secondary)',
                        transition: 'color var(--transition-fast)'
                    }}
                    title={isCollapsed ? 'Expand legend' : 'Collapse legend'}
                    onMouseEnter={(e) => e.currentTarget.style.color = 'var(--color-text-primary)'}
                    onMouseLeave={(e) => e.currentTarget.style.color = 'var(--color-text-secondary)'}
                >
                    {isCollapsed ? '▼' : '▲'}
                </button>
            </div>

            {!isCollapsed && (
                <>
                    {/* View Mode Information */}
                    <div style={{ marginBottom: '12px' }}>
                        <div style={{ fontWeight: '600', marginBottom: '6px' }}>
                            Current View: {viewMode === 'clusters' ? '📊 Cluster View' : '🔗 Similarity Network'}
                        </div>
                        <div style={{ fontSize: '11px', color: 'var(--color-text-secondary)' }}>
                            {viewMode === 'clusters'
                                ? 'Papers grouped by topic clusters with no edges'
                                : 'Papers connected by similarity relationships'
                            }
                        </div>
                    </div>

                    {/* Special Node Information */}
                    {selectedPaperId && (
                        <div style={{ marginBottom: '12px' }}>
                            <div style={{ fontWeight: '600', marginBottom: '6px' }}>Special Nodes:</div>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
                                <div style={{
                                    width: '20px',
                                    height: '20px',
                                    borderRadius: '50%',
                                    backgroundColor: 'var(--color-node-selected)',
                                    border: '2px solid var(--color-node-border)'
                                }}></div>
                                <span>Selected Paper (Central Node)</span>
                            </div>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                <div style={{
                                    width: '16px',
                                    height: '16px',
                                    borderRadius: '50%',
                                    backgroundColor: 'var(--color-cluster-1)',
                                    border: '1px solid var(--color-node-border)'
                                }}></div>
                                <span>Similar Papers</span>
                            </div>
                        </div>
                    )}

                    {/* Cluster Information */}
                    {viewMode === 'clusters' && Object.keys(clusterInfo).length > 0 && (
                        <div style={{ marginBottom: '12px' }}>
                            <div style={{ fontWeight: '600', marginBottom: '6px' }}>
                                Clusters ({Object.keys(clusterInfo).length}):
                            </div>
                            {Object.entries(clusterInfo)
                                .sort(([, a], [, b]) => b.count - a.count)
                                .map(([clusterId, info]) => (
                                    <div key={clusterId} style={{ marginBottom: '4px' }}>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                            <div style={{
                                                width: '12px',
                                                height: '12px',
                                                borderRadius: '50%',
                                                backgroundColor: clusterColors[parseInt(clusterId)] || 'var(--color-node-default)'
                                            }}></div>
                                            <span>Cluster {clusterId}: {info.count} papers</span>
                                        </div>
                                    </div>
                                ))}
                        </div>
                    )}

                    {/* Subject Areas */}
                    {Object.keys(subjectAreaInfo).length > 0 && (
                        <div style={{ marginBottom: '12px' }}>
                            <div style={{ fontWeight: '600', marginBottom: '6px' }}>
                                Subject Areas ({Object.keys(subjectAreaInfo).length}):
                            </div>
                            <div style={{ maxHeight: '120px', overflowY: 'auto' }}>
                                {Object.entries(subjectAreaInfo)
                                    .sort(([, a], [, b]) => b - a)
                                    .slice(0, 10) // Show top 10
                                    .map(([area, count]) => (
                                        <div key={area} style={{
                                            marginBottom: '2px',
                                            fontSize: '11px',
                                            color: 'var(--color-text-secondary)'
                                        }}>
                                            {area}: {count} papers
                                        </div>
                                    ))}
                                {Object.keys(subjectAreaInfo).length > 10 && (
                                    <div style={{ fontSize: '11px', color: 'var(--color-text-muted)', fontStyle: 'italic' }}>
                                        ... and {Object.keys(subjectAreaInfo).length - 10} more
                                    </div>
                                )}
                            </div>
                        </div>
                    )}

                    {/* Statistics */}
                    <div style={{ marginBottom: '12px' }}>
                        <div style={{ fontWeight: '600', marginBottom: '6px' }}>Statistics:</div>
                        <div style={{ fontSize: '11px', color: 'var(--color-text-secondary)' }}>
                            <div>Total Papers: {data.nodes.length}</div>
                            <div>Total Edges: {data.edges.length}</div>
                            {viewMode === 'similarity' && (
                                <div>Avg Similarity: {
                                    data.edges.length > 0
                                        ? (data.edges.reduce((sum, edge) => sum + edge.similarity, 0) / data.edges.length).toFixed(3)
                                        : 'N/A'
                                }</div>
                            )}
                        </div>
                    </div>

                    {/* Instructions */}
                    <div style={{ fontSize: '11px', color: 'var(--color-text-muted)' }}>
                        <div style={{ fontWeight: '600', marginBottom: '4px' }}>Instructions:</div>
                        <div>• Click nodes to explore</div>
                        <div>• Drag to move nodes</div>
                        <div>• Scroll to zoom</div>
                        <div>• Hover for details</div>
                    </div>
                </>
            )}
        </div>
    );
};

export default GraphLegend;
