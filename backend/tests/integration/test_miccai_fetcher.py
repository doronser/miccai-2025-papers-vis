"""
Integration tests for MICCAI data fetching.

Tests the actual fetching from https://papers.miccai.org/miccai-2025/
These tests will fail initially (TDD) and pass once the implementation works.
"""

import json
import tempfile
from pathlib import Path
from unittest.mock import Mock, patch

import pytest
import requests
from bs4 import BeautifulSoup

from src.lib.miccai_scraper import MICCAIScraper, MICCAIConfig, RawPaperData


class TestMICCAIFetcherIntegration:
    """Integration tests for MICCAI paper fetching from real website."""

    @pytest.fixture
    def scraper(self):
        """Create a scraper instance for testing."""
        config = MICCAIConfig(
            delay_seconds=0.1,  # Faster for tests
            timeout_seconds=10,
            max_retries=1
        )
        return MICCAIScraper(config)

    @pytest.fixture
    def mock_miccai_homepage(self):
        """Mock HTML content for MICCAI homepage."""
        return """
        <html>
        <head><title>MICCAI 2025</title></head>
        <body>
            <div class="papers">
                <a href="/paper/001/">Deep Learning for Medical Image Segmentation</a>
                <a href="/paper/002/">Novel Approach to Brain Tumor Detection</a>
                <a href="/paper/003/">AI in Radiology: Current State</a>
            </div>
        </body>
        </html>
        """

    @pytest.fixture
    def mock_paper_page(self):
        """Mock HTML content for a single paper page."""
        return """
        <html>
        <head><title>Deep Learning for Medical Image Segmentation</title></head>
        <body>
            <h1>Deep Learning for Medical Image Segmentation</h1>
            <div class="authors">John Smith, Jane Doe, Bob Wilson</div>
            <div class="abstract">
                This paper presents a novel deep learning approach for medical image segmentation
                using convolutional neural networks. The method achieves state-of-the-art results
                on multiple datasets including brain MRI and cardiac CT scans.
            </div>
            <a href="/papers/001.pdf">Download PDF</a>
            <a href="https://github.com/author/segmentation-code">Source Code</a>
            <div class="subject-areas">Computer Vision, Machine Learning, Medical Imaging</div>
        </body>
        </html>
        """

    def test_scraper_initialization(self, scraper):
        """Test that scraper initializes correctly."""
        assert scraper.config.base_url == "https://papers.miccai.org/miccai-2025/"
        assert scraper.config.delay_seconds == 0.1
        assert scraper.session.headers['User-Agent'].startswith('MICCAI-Papers-Scraper')

    @pytest.mark.integration
    def test_fetch_real_miccai_homepage(self, scraper):
        """Test fetching the actual MICCAI homepage."""
        # This test connects to the real website
        soup = scraper.fetch_page(scraper.config.base_url)

        # Basic validation that we got a valid response
        assert soup is not None, "Should successfully fetch MICCAI homepage"
        assert soup.find('title') is not None, "Page should have a title"

        # Look for evidence this is the MICCAI 2025 page
        page_text = soup.get_text().lower()
        assert any(term in page_text for term in ['miccai', '2025', 'papers', 'conference']), \
            "Page should contain MICCAI 2025 related content"

    @pytest.mark.integration
    def test_discover_paper_urls_real_site(self, scraper):
        """Test discovering paper URLs from real MICCAI site."""
        # This test connects to the real website
        paper_urls = scraper.discover_paper_urls()

        # Validate we found some URLs
        assert len(paper_urls) > 0, "Should discover at least some paper URLs"

        # Validate URLs are properly formatted
        for url in paper_urls[:5]:  # Check first 5 to avoid overwhelming tests
            assert url.startswith(('http://', 'https://')), f"URL should be absolute: {url}"

    @pytest.mark.unit
    def test_discover_paper_urls_with_mock(self, scraper, mock_miccai_homepage):
        """Test URL discovery with mocked content."""
        with patch.object(scraper, 'fetch_page') as mock_fetch:
            mock_fetch.return_value = BeautifulSoup(mock_miccai_homepage, 'html.parser')

            paper_urls = scraper.discover_paper_urls("https://example.com/miccai-2025/")

            assert len(paper_urls) == 3
            assert "https://example.com/paper/001/" in paper_urls
            assert "https://example.com/paper/002/" in paper_urls
            assert "https://example.com/paper/003/" in paper_urls

    @pytest.mark.unit
    def test_scrape_paper_with_mock(self, scraper, mock_paper_page):
        """Test scraping a single paper with mocked content."""
        test_url = "https://example.com/paper/001/"

        with patch.object(scraper, 'fetch_page') as mock_fetch:
            mock_fetch.return_value = BeautifulSoup(mock_paper_page, 'html.parser')

            paper_data = scraper.scrape_paper(test_url)

            assert paper_data is not None
            assert paper_data.title == "Deep Learning for Medical Image Segmentation"
            assert len(paper_data.authors) == 3
            assert "John Smith" in paper_data.authors
            assert "Jane Doe" in paper_data.authors
            assert "Bob Wilson" in paper_data.authors
            assert "novel deep learning approach" in paper_data.abstract
            assert paper_data.pdf_url.endswith('.pdf')
            assert any(link['type'] == 'github' for link in paper_data.external_links)

    @pytest.mark.unit
    def test_save_raw_data(self, scraper):
        """Test saving raw paper data to JSON file."""
        # Create sample paper data
        papers = [
            RawPaperData(
                title="Test Paper 1",
                authors=["Author One", "Author Two"],
                abstract="Test abstract",
                url="https://example.com/paper1",
                pdf_url="https://example.com/paper1.pdf"
            ),
            RawPaperData(
                title="Test Paper 2",
                authors=["Author Three"],
                abstract="Another test abstract",
                url="https://example.com/paper2"
            )
        ]

        # Save to temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            temp_path = f.name

        try:
            scraper.save_raw_data(papers, temp_path)

            # Verify file was created and contains correct data
            assert Path(temp_path).exists()

            with open(temp_path, 'r', encoding='utf-8') as f:
                loaded_data = json.load(f)

            assert len(loaded_data) == 2
            assert loaded_data[0]['title'] == "Test Paper 1"
            assert len(loaded_data[0]['authors']) == 2
            assert loaded_data[1]['title'] == "Test Paper 2"

        finally:
            # Clean up
            Path(temp_path).unlink(missing_ok=True)

    @pytest.mark.integration
    @pytest.mark.slow
    def test_scrape_sample_papers_real_site(self, scraper):
        """Test scraping a few papers from the real MICCAI site."""
        # This test connects to the real website and may be slow
        paper_urls = scraper.discover_paper_urls()

        if len(paper_urls) == 0:
            pytest.skip("No paper URLs found on the real site")

        # Try to scrape first few papers
        sample_urls = paper_urls[:3]  # Limit to 3 papers for testing
        papers = []

        for url in sample_urls:
            paper_data = scraper.scrape_paper(url)
            if paper_data and paper_data.title:
                papers.append(paper_data)

        # Validate results
        assert len(papers) > 0, "Should successfully scrape at least one paper"

        for paper in papers:
            assert paper.title is not None and paper.title.strip() != "", \
                "Each paper should have a non-empty title"
            assert paper.url is not None, "Each paper should have a URL"
            assert paper.scraped_at is not None, "Each paper should have a timestamp"

    @pytest.mark.unit
    def test_network_error_handling(self, scraper):
        """Test handling of network errors during scraping."""
        # Test with invalid URL
        paper_data = scraper.scrape_paper("https://invalid-url-that-does-not-exist.com")
        assert paper_data is None, "Should return None for invalid URLs"

        # Test timeout handling
        with patch('requests.Session.get') as mock_get:
            mock_get.side_effect = requests.Timeout("Request timed out")
            paper_data = scraper.scrape_paper("https://example.com/paper")
            assert paper_data is None, "Should return None for timeout errors"

    def test_cli_interface_exists(self):
        """Test that the CLI interface is properly defined."""
        # Import the CLI function to ensure it exists
        from src.lib.miccai_scraper import main

        # Check that it's a Click command
        assert hasattr(main, 'callback'), "Should be a Click command"
        assert hasattr(main, 'params'), "Should have parameters defined"

    @pytest.mark.integration
    def test_end_to_end_scraping_workflow(self, scraper):
        """Test the complete scraping workflow with limited scope."""
        # This is a comprehensive test that exercises the full pipeline
        # but limits scope to avoid overwhelming the target site

        with tempfile.TemporaryDirectory() as temp_dir:
            output_file = Path(temp_dir) / "test_papers.json"

            # Mock the discover_paper_urls to return just one URL for testing
            with patch.object(scraper, 'discover_paper_urls') as mock_discover:
                mock_discover.return_value = [scraper.config.base_url]

                # Mock scrape_paper to return sample data
                with patch.object(scraper, 'scrape_paper') as mock_scrape:
                    mock_scrape.return_value = RawPaperData(
                        title="Integration Test Paper",
                        authors=["Test Author"],
                        abstract="Test abstract for integration",
                        url=scraper.config.base_url
                    )

                    # Run the full workflow
                    papers = scraper.scrape_all_papers()
                    scraper.save_raw_data(papers, str(output_file))

                    # Verify results
                    assert len(papers) == 1
                    assert papers[0].title == "Integration Test Paper"
                    assert output_file.exists()

                    # Verify saved data
                    with open(output_file, 'r') as f:
                        saved_data = json.load(f)
                    assert len(saved_data) == 1
                    assert saved_data[0]['title'] == "Integration Test Paper"