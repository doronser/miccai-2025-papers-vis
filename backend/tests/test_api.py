import pytest
from fastapi.testclient import TestClient
from main import app

client = TestClient(app)


def test_root_endpoint():
    response = client.get("/")
    assert response.status_code == 200
    assert "message" in response.json()


def test_health_endpoint():
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json() == {"status": "healthy"}


def test_papers_endpoint():
    response = client.get("/api/papers/")
    assert response.status_code == 200
    data = response.json()
    assert isinstance(data, list)
    # Should have some papers loaded
    assert len(data) > 0


def test_papers_pagination():
    response = client.get("/api/papers/", params={"limit": 5, "offset": 0})
    assert response.status_code == 200
    data = response.json()
    assert len(data) <= 5


def test_search_papers():
    response = client.get("/api/papers/search", params={"q": "medical"})
    assert response.status_code == 200
    data = response.json()
    assert isinstance(data, list)


def test_search_papers_empty_query():
    response = client.get("/api/papers/search", params={"q": ""})
    assert response.status_code == 422  # Validation error


def test_get_paper_by_id():
    # First get a paper ID from the papers list
    papers_response = client.get("/api/papers/", params={"limit": 1})
    assert papers_response.status_code == 200
    papers = papers_response.json()

    if papers:
        paper_id = papers[0]["id"]
        response = client.get(f"/api/papers/{paper_id}")
        assert response.status_code == 200
        data = response.json()
        assert data["id"] == paper_id
        assert "title" in data
        assert "abstract" in data
        assert "authors" in data


def test_get_nonexistent_paper():
    response = client.get("/api/papers/nonexistent-id")
    assert response.status_code == 404


def test_similar_papers():
    # First get a paper ID
    papers_response = client.get("/api/papers/", params={"limit": 1})
    papers = papers_response.json()

    if papers:
        paper_id = papers[0]["id"]
        response = client.get(f"/api/papers/{paper_id}/similar")
        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)


def test_similar_papers_nonexistent():
    response = client.get("/api/papers/nonexistent-id/similar")
    assert response.status_code == 404


def test_dataset_stats():
    response = client.get("/api/papers/stats/summary")
    assert response.status_code == 200
    data = response.json()
    assert "total_papers" in data
    assert "total_authors" in data
    assert "subject_areas" in data
    assert isinstance(data["total_papers"], int)
    assert isinstance(data["total_authors"], int)


def test_graph_data():
    response = client.get("/api/papers/graph/data")
    assert response.status_code == 200
    data = response.json()
    assert "nodes" in data
    assert "edges" in data
    assert isinstance(data["nodes"], list)
    assert isinstance(data["edges"], list)


def test_graph_data_with_params():
    response = client.get("/api/papers/graph/data", params={
        "similarity_threshold": 0.8,
        "max_edges": 100
    })
    assert response.status_code == 200
    data = response.json()
    assert len(data["edges"]) <= 100


def test_paper_clusters():
    response = client.get("/api/papers/clusters/")
    assert response.status_code == 200
    data = response.json()
    assert isinstance(data, dict)
    # Should have cluster data
    if data:
        cluster_id = list(data.keys())[0]
        assert "papers" in data[cluster_id]
        assert "size" in data[cluster_id]


def test_paper_clusters_with_params():
    response = client.get("/api/papers/clusters/", params={"n_clusters": 5})
    assert response.status_code == 200
    data = response.json()
    # Should have at most 5 clusters
    assert len(data) <= 5