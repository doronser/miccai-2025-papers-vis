"""Contract test for GET /api/v1/papers endpoint."""

import pytest
from httpx import AsyncClient
from fastapi import status

from backend.main import app


class TestPapersGetContract:
    """Test contract for GET /api/v1/papers endpoint."""

    @pytest.mark.asyncio
    async def test_get_papers_success_schema(self):
        """Test GET /api/v1/papers returns correct schema on success."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers")

        assert response.status_code == status.HTTP_200_OK

        data = response.json()

        # Validate response structure
        assert "papers" in data
        assert "total" in data
        assert "page_info" in data
        assert isinstance(data["papers"], list)
        assert isinstance(data["total"], int)
        assert isinstance(data["page_info"], dict)

        # Validate page_info structure
        page_info = data["page_info"]
        assert "limit" in page_info
        assert "offset" in page_info
        assert "has_next" in page_info
        assert "has_previous" in page_info

    @pytest.mark.asyncio
    async def test_get_papers_with_search_filter(self):
        """Test GET /api/v1/papers with search parameter."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers", params={"search": "deep learning"})

        assert response.status_code == status.HTTP_200_OK
        data = response.json()
        assert "papers" in data

    @pytest.mark.asyncio
    async def test_get_papers_with_subject_areas_filter(self):
        """Test GET /api/v1/papers with subject_areas parameter."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers", params={"subject_areas": "Computer Vision,Machine Learning"})

        assert response.status_code == status.HTTP_200_OK
        data = response.json()
        assert "papers" in data

    @pytest.mark.asyncio
    async def test_get_papers_with_pagination(self):
        """Test GET /api/v1/papers with limit and offset."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers", params={"limit": 50, "offset": 10})

        assert response.status_code == status.HTTP_200_OK
        data = response.json()
        assert data["page_info"]["limit"] == 50
        assert data["page_info"]["offset"] == 10

    @pytest.mark.asyncio
    async def test_get_papers_validation_error(self):
        """Test GET /api/v1/papers with invalid parameters returns 400."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers", params={"limit": -1})

        assert response.status_code == status.HTTP_400_BAD_REQUEST

        data = response.json()
        assert "error" in data
        assert "message" in data

    @pytest.mark.asyncio
    async def test_paper_schema_when_present(self):
        """Test individual paper schema when papers are returned."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get("/api/v1/papers", params={"limit": 1})

        assert response.status_code == status.HTTP_200_OK
        data = response.json()

        if data["papers"]:
            paper = data["papers"][0]
            # Validate required paper fields per OpenAPI spec
            required_fields = ["id", "title", "abstract", "authors", "subject_areas", "publication_date"]
            for field in required_fields:
                assert field in paper
                assert paper[field] is not None

            # Validate field types
            assert isinstance(paper["id"], str)
            assert isinstance(paper["title"], str)
            assert isinstance(paper["abstract"], str)
            assert isinstance(paper["authors"], list)
            assert isinstance(paper["subject_areas"], list)
            assert isinstance(paper["publication_date"], str)

            # Validate authors structure
            if paper["authors"]:
                author = paper["authors"][0]
                assert "name" in author
                assert isinstance(author["name"], str)