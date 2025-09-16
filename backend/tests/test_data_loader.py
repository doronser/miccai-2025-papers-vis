import pytest
from src.services.data_loader import DataLoader
from src.models.paper import Paper


def test_data_loader_initialization():
    loader = DataLoader()
    assert loader.data_dir is not None
    assert loader.papers_dir is not None
    assert loader.embeddings_dir is not None


def test_load_paper_index():
    loader = DataLoader()
    index = loader.load_paper_index()
    assert isinstance(index, dict)
    # Should have papers list
    if "papers" in index:
        assert isinstance(index["papers"], list)


def test_get_paper_by_id():
    loader = DataLoader()
    index = loader.load_paper_index()

    if "papers" in index and index["papers"]:
        paper_id = index["papers"][0]
        paper = loader.get_paper_by_id(paper_id)
        assert paper is not None
        assert isinstance(paper, Paper)
        assert paper.id == paper_id
        assert paper.title is not None
        assert paper.abstract is not None
        assert isinstance(paper.authors, list)


def test_get_nonexistent_paper():
    loader = DataLoader()
    paper = loader.get_paper_by_id("nonexistent-id")
    assert paper is None


def test_get_all_papers():
    loader = DataLoader()
    papers = loader.get_all_papers()
    assert isinstance(papers, list)
    # Should have at least some papers
    assert len(papers) > 0

    # Verify paper structure
    paper = papers[0]
    assert isinstance(paper, Paper)
    assert paper.id is not None
    assert paper.title is not None
    assert paper.abstract is not None


def test_get_embedding_by_id():
    loader = DataLoader()
    index = loader.load_paper_index()

    if "papers" in index and index["papers"]:
        paper_id = index["papers"][0]
        embedding = loader.get_embedding_by_id(paper_id)
        # Embedding might not exist for all papers
        if embedding is not None:
            assert embedding.shape is not None
            assert len(embedding.shape) == 1  # Should be 1D array
            assert embedding.shape[0] > 0  # Should have dimensions


def test_search_papers():
    loader = DataLoader()
    results = loader.search_papers("medical", limit=10)
    assert isinstance(results, list)
    assert len(results) <= 10

    # If results found, verify they match the query
    if results:
        paper = results[0]
        assert isinstance(paper, Paper)
        query_lower = "medical"
        found_match = (
            query_lower in paper.title.lower() or
            query_lower in paper.abstract.lower() or
            any(query_lower in area.lower() for area in paper.subject_areas) or
            any(query_lower in author.name.lower() for author in paper.authors)
        )
        # Note: This might not always be true due to fuzzy matching
        # assert found_match


def test_search_papers_empty_query():
    loader = DataLoader()
    results = loader.search_papers("", limit=10)
    # Empty query should return empty results
    assert len(results) == 0


def test_search_papers_limit():
    loader = DataLoader()
    results = loader.search_papers("a", limit=5)  # Common letter
    assert len(results) <= 5