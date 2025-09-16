import pytest
import numpy as np
from src.services.data_loader import DataLoader
from src.services.similarity import SimilarityService
from src.models.paper import PaperSimilarity, GraphData


def test_similarity_service_initialization():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)
    assert similarity_service.data_loader == loader


def test_find_similar_papers():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)

    # Get a paper ID that should have an embedding
    index = loader.load_paper_index()
    if "papers" in index and index["papers"]:
        paper_id = index["papers"][0]

        # Check if embedding exists first
        embedding = loader.get_embedding_by_id(paper_id)
        if embedding is not None:
            similar_papers = similarity_service.find_similar_papers(paper_id, limit=5)
            assert isinstance(similar_papers, list)
            assert len(similar_papers) <= 5

            # Verify structure of similar papers
            for similar in similar_papers:
                assert isinstance(similar, PaperSimilarity)
                assert similar.paper_id != paper_id  # Should not include self
                assert 0 <= similar.similarity_score <= 1
                assert similar.paper is not None


def test_find_similar_papers_nonexistent():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)

    similar_papers = similarity_service.find_similar_papers("nonexistent-id")
    assert isinstance(similar_papers, list)
    assert len(similar_papers) == 0


def test_generate_graph_data():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)

    graph_data = similarity_service.generate_graph_data(
        similarity_threshold=0.8,
        max_edges=100
    )

    assert isinstance(graph_data, GraphData)
    assert isinstance(graph_data.nodes, list)
    assert isinstance(graph_data.edges, list)
    assert len(graph_data.edges) <= 100

    # Verify node structure
    if graph_data.nodes:
        node = graph_data.nodes[0]
        assert node.id is not None
        assert node.title is not None
        assert isinstance(node.authors, list)
        assert isinstance(node.subject_areas, list)

    # Verify edge structure
    if graph_data.edges:
        edge = graph_data.edges[0]
        assert edge.source is not None
        assert edge.target is not None
        assert 0 <= edge.similarity <= 1


def test_get_paper_clusters():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)

    # Only test if we have embeddings
    embeddings = loader.get_all_embeddings()
    if embeddings and len(embeddings) >= 5:  # Need enough data for clustering
        clusters = similarity_service.get_paper_clusters(n_clusters=3)
        assert isinstance(clusters, dict)
        assert len(clusters) <= 3  # Should have at most 3 clusters

        # Verify cluster structure
        total_papers = 0
        for cluster_id, paper_ids in clusters.items():
            assert isinstance(paper_ids, list)
            total_papers += len(paper_ids)

        # All papers should be assigned to clusters
        assert total_papers == len(embeddings)


def test_clustering_with_insufficient_data():
    loader = DataLoader()
    similarity_service = SimilarityService(loader)

    # Mock empty embeddings
    original_get_embeddings = similarity_service._get_embeddings
    similarity_service._get_embeddings = lambda: {}

    clusters = similarity_service.get_paper_clusters(n_clusters=5)
    assert isinstance(clusters, dict)
    assert len(clusters) == 0

    # Restore original method
    similarity_service._get_embeddings = original_get_embeddings