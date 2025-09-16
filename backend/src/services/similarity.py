import numpy as np
from sklearn.metrics.pairwise import cosine_similarity
from sklearn.cluster import KMeans
from typing import Dict, List, Tuple, Optional
from ..models.paper import Paper, PaperSimilarity, GraphNode, GraphEdge, GraphData
from .data_loader import DataLoader


class SimilarityService:
    def __init__(self, data_loader: DataLoader):
        self.data_loader = data_loader
        self._all_embeddings: Optional[Dict[str, np.ndarray]] = None

    def _get_embeddings(self) -> Dict[str, np.ndarray]:
        """Lazy load all embeddings"""
        if self._all_embeddings is None:
            self._all_embeddings = self.data_loader.get_all_embeddings()
        return self._all_embeddings

    def find_similar_papers(self, paper_id: str, limit: int = 10) -> List[PaperSimilarity]:
        """Find papers similar to the given paper using cosine similarity"""
        target_embedding = self.data_loader.get_embedding_by_id(paper_id)
        if target_embedding is None:
            return []

        all_embeddings = self._get_embeddings()
        similarities = []

        for other_id, other_embedding in all_embeddings.items():
            if other_id == paper_id:
                continue

            similarity = cosine_similarity([target_embedding], [other_embedding])[0][0]
            similarities.append((other_id, float(similarity)))

        # Sort by similarity descending
        similarities.sort(key=lambda x: x[1], reverse=True)

        # Get top similar papers with their details
        result = []
        for other_id, score in similarities[:limit]:
            paper = self.data_loader.get_paper_by_id(other_id)
            if paper:
                result.append(PaperSimilarity(
                    paper_id=other_id,
                    similarity_score=score,
                    paper=paper
                ))

        return result

    def generate_graph_data(self, similarity_threshold: float = 0.7, max_edges: int = 1000, sample_size: Optional[int] = None) -> GraphData:
        """Generate graph data with papers as nodes and similarities as edges"""
        all_embeddings = self._get_embeddings()
        paper_ids = list(all_embeddings.keys())

        # Sample papers if requested for better performance
        if sample_size and sample_size < len(paper_ids):
            import random
            paper_ids = random.sample(paper_ids, sample_size)

        # Create nodes
        nodes = []
        for paper_id in paper_ids:
            paper = self.data_loader.get_paper_by_id(paper_id)
            if paper:
                node = GraphNode(
                    id=paper_id,
                    title=paper.title,
                    authors=[author.name for author in paper.authors],
                    subject_areas=paper.subject_areas
                )
                nodes.append(node)

        # Calculate similarity matrix only for selected papers
        selected_embeddings = {pid: all_embeddings[pid] for pid in paper_ids if pid in all_embeddings}
        embeddings_matrix = np.array([selected_embeddings[pid] for pid in paper_ids])
        similarity_matrix = cosine_similarity(embeddings_matrix)

        # Create edges based on similarity threshold
        edges = []
        for i, source_id in enumerate(paper_ids):
            for j, target_id in enumerate(paper_ids):
                if i < j and similarity_matrix[i][j] > similarity_threshold:
                    edge = GraphEdge(
                        source=source_id,
                        target=target_id,
                        similarity=float(similarity_matrix[i][j])
                    )
                    edges.append(edge)

        # Limit edges for performance
        edges.sort(key=lambda x: x.similarity, reverse=True)
        edges = edges[:max_edges]

        # Perform clustering
        n_clusters = min(10, len(paper_ids) // 20)  # Adaptive cluster count
        if n_clusters > 1:
            kmeans = KMeans(n_clusters=n_clusters, random_state=42)
            cluster_labels = kmeans.fit_predict(embeddings_matrix)

            # Add cluster information to nodes
            for i, node in enumerate(nodes):
                node.cluster = int(cluster_labels[i])

            clusters = {
                str(i): {
                    "id": i,
                    "papers": [paper_ids[j] for j in range(len(paper_ids)) if cluster_labels[j] == i],
                    "center": kmeans.cluster_centers_[i].tolist()
                }
                for i in range(n_clusters)
            }
        else:
            clusters = {}

        return GraphData(nodes=nodes, edges=edges, clusters=clusters)

    def get_paper_clusters(self, n_clusters: int = 10) -> Dict[str, List[str]]:
        """Cluster papers based on their embeddings"""
        all_embeddings = self._get_embeddings()
        paper_ids = list(all_embeddings.keys())

        if len(paper_ids) == 0:
            return {}

        if len(paper_ids) < n_clusters:
            n_clusters = len(paper_ids)

        embeddings_matrix = np.array([all_embeddings[pid] for pid in paper_ids])

        if embeddings_matrix.size == 0:
            return {}

        kmeans = KMeans(n_clusters=n_clusters, random_state=42)
        cluster_labels = kmeans.fit_predict(embeddings_matrix)

        clusters = {}
        for i, paper_id in enumerate(paper_ids):
            cluster_id = str(cluster_labels[i])
            if cluster_id not in clusters:
                clusters[cluster_id] = []
            clusters[cluster_id].append(paper_id)

        return clusters
