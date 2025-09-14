"""Contract test for GET /api/v1/papers/{paper_id} endpoint."""

import pytest
from httpx import AsyncClient
from fastapi import status

from backend.main import app


class TestPaperDetailContract:
    """Test contract for GET /api/v1/papers/{paper_id} endpoint."""

    @pytest.mark.asyncio
    async def test_get_paper_success_schema(self):
        """Test GET /api/v1/papers/{paper_id} returns correct paper schema."""
        paper_id = "paper-001"
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get(f"/api/v1/papers/{paper_id}")

        # This should return 200 when implementation exists
        assert response.status_code == status.HTTP_200_OK

        paper = response.json()

        # Validate required paper fields per OpenAPI spec
        required_fields = ["id", "title", "abstract", "authors", "subject_areas", "publication_date"]
        for field in required_fields:
            assert field in paper
            assert paper[field] is not None

        # Validate field types and constraints
        assert isinstance(paper["id"], str)
        assert isinstance(paper["title"], str)
        assert len(paper["title"]) <= 500  # maxLength constraint
        assert isinstance(paper["abstract"], str)
        assert len(paper["abstract"]) <= 5000  # maxLength constraint
        assert isinstance(paper["authors"], list)
        assert len(paper["authors"]) >= 1  # minItems constraint
        assert isinstance(paper["subject_areas"], list)
        assert len(paper["subject_areas"]) >= 1  # minItems constraint
        assert isinstance(paper["publication_date"], str)

        # Validate paper ID matches requested ID
        assert paper["id"] == paper_id

    @pytest.mark.asyncio
    async def test_get_paper_authors_schema(self):
        """Test paper authors have correct schema."""
        paper_id = "paper-001"
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get(f"/api/v1/papers/{paper_id}")

        assert response.status_code == status.HTTP_200_OK
        paper = response.json()

        # Validate authors structure
        assert len(paper["authors"]) >= 1
        for author in paper["authors"]:
            assert "name" in author
            assert isinstance(author["name"], str)
            assert len(author["name"]) <= 200  # maxLength constraint

            # Optional fields
            if "affiliation" in author:
                assert isinstance(author["affiliation"], str)
                assert len(author["affiliation"]) <= 300
            if "email" in author:
                assert isinstance(author["email"], str)
                # Basic email format validation
                assert "@" in author["email"]

    @pytest.mark.asyncio
    async def test_get_paper_external_links_schema(self):
        """Test paper external links have correct schema when present."""
        paper_id = "paper-001"
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get(f"/api/v1/papers/{paper_id}")

        assert response.status_code == status.HTTP_200_OK
        paper = response.json()

        if "external_links" in paper and paper["external_links"]:
            for link in paper["external_links"]:
                assert "type" in link
                assert "url" in link
                assert link["type"] in ["github", "dataset", "website", "pdf"]
                assert isinstance(link["url"], str)
                assert link["url"].startswith(("http://", "https://"))

                if "description" in link:
                    assert isinstance(link["description"], str)
                    assert len(link["description"]) <= 200

    @pytest.mark.asyncio
    async def test_get_paper_not_found(self):
        """Test GET /api/v1/papers/{paper_id} returns 404 for non-existent paper."""
        paper_id = "non-existent-paper"
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get(f"/api/v1/papers/{paper_id}")

        assert response.status_code == status.HTTP_404_NOT_FOUND

        data = response.json()
        assert "error" in data
        assert "message" in data
        assert isinstance(data["error"], str)
        assert isinstance(data["message"], str)

    @pytest.mark.asyncio
    async def test_get_paper_invalid_id_format(self):
        """Test GET /api/v1/papers/{paper_id} with various ID formats."""
        test_cases = [
            "paper-123",      # valid format
            "123",            # number only
            "paper_456",      # underscore
            "PAPER-789",      # uppercase
        ]

        async with AsyncClient(app=app, base_url="http://test") as client:
            for paper_id in test_cases:
                response = await client.get(f"/api/v1/papers/{paper_id}")
                # Should either return 200 (found) or 404 (not found)
                # but never 400 (bad request) for valid ID formats
                assert response.status_code in [status.HTTP_200_OK, status.HTTP_404_NOT_FOUND]