"""
Integration tests for paper parsing with real MICCAI data samples.

Tests the parsing of HTML content from MICCAI into structured Paper entities.
These tests will fail initially (TDD) and pass once the implementation works.
"""

import json
import tempfile
from pathlib import Path

import pytest
from src.lib.paper_parser import PaperParser, Paper, Author, ExternalLink


class TestPaperParserIntegration:
    """Integration tests for paper parsing functionality."""

    @pytest.fixture
    def parser(self):
        """Create a parser instance for testing."""
        return PaperParser()

    @pytest.fixture
    def sample_miccai_html(self):
        """Sample HTML content similar to what MICCAI papers might have."""
        return """
        <html>
        <head><title>Deep Learning for Medical Image Segmentation - MICCAI 2025</title></head>
        <body>
            <h1>Deep Learning for Medical Image Segmentation</h1>
            <div class="authors">
                John Smith (University of Example), Jane Doe (Medical Center),
                Bob Wilson (bob.wilson@example.com)
            </div>
            <div class="abstract">
                This paper presents a novel deep learning approach for medical image segmentation
                using convolutional neural networks. The proposed method incorporates attention
                mechanisms and multi-scale feature extraction to achieve state-of-the-art results
                on multiple datasets including brain MRI and cardiac CT scans. We demonstrate
                significant improvements in segmentation accuracy and computational efficiency
                compared to existing methods. The approach is validated on three public datasets
                and shows consistent performance across different imaging modalities.
            </div>
            <div class="subject-areas">Computer Vision, Machine Learning, Medical Imaging</div>
            <div class="links">
                <a href="/papers/001.pdf">Download PDF</a>
                <a href="https://github.com/author/segmentation-code">Source Code</a>
                <a href="https://dataset-repository.com/medical-images">Medical Image Dataset</a>
                <a href="https://example.com/project-website">Project Website</a>
            </div>
        </body>
        </html>
        """

    @pytest.fixture
    def minimal_miccai_html(self):
        """Minimal HTML that should still parse successfully."""
        return """
        <html>
        <body>
            <h1>AI in Radiology: Current Applications</h1>
            <p>This research explores the current applications of artificial intelligence
               in radiology, examining how machine learning algorithms are being integrated
               into clinical workflows to assist radiologists in diagnosis and treatment planning.</p>
            <div class="authors">Dr. Sarah Johnson, Michael Chen</div>
        </body>
        </html>
        """

    @pytest.fixture
    def complex_authors_html(self):
        """HTML with complex author formatting."""
        return """
        <html>
        <body>
            <h1>Novel Approach to Brain Tumor Detection</h1>
            <div class="authors">
                Dr. Alice Smith¹ (alice@med.edu), Prof. Bob Jones² and Carol Wilson³;
                David Lee¹ | Emma Davis² (emma.davis@hospital.org)
            </div>
            <div class="abstract">
                This study introduces a novel machine learning approach for automatic brain tumor
                detection in MRI scans. Using a combination of convolutional neural networks and
                traditional image processing techniques, we achieve 95% accuracy on a dataset of
                1000 brain MRI images. The method shows promise for clinical implementation.
            </div>
        </body>
        </html>
        """

    @pytest.fixture
    def sample_raw_papers_json(self, sample_miccai_html, minimal_miccai_html):
        """Sample raw papers JSON data for testing."""
        return [
            {
                "title": "Deep Learning for Medical Image Segmentation",
                "url": "https://papers.miccai.org/paper/001/",
                "html_content": sample_miccai_html,
                "scraped_at": "2025-09-14 10:00:00",
                "source_url": "https://papers.miccai.org/paper/001/"
            },
            {
                "title": "AI in Radiology",
                "url": "https://papers.miccai.org/paper/002/",
                "html_content": minimal_miccai_html,
                "scraped_at": "2025-09-14 10:01:00",
                "source_url": "https://papers.miccai.org/paper/002/"
            }
        ]

    def test_parser_initialization(self, parser):
        """Test that parser initializes correctly."""
        assert isinstance(parser, PaperParser)
        assert isinstance(parser.default_subject_areas, list)
        assert len(parser.default_subject_areas) > 0

    def test_parse_authors_basic(self, parser):
        """Test basic author parsing with different formats."""
        test_cases = [
            ("John Smith, Jane Doe", 2),
            ("Dr. Alice Johnson; Bob Wilson", 2),
            ("Sarah Lee and Michael Chen", 2),
            ("Emma Davis | David Kim", 2),
            ("Single Author", 1),
            ("", 1),  # Should return Unknown Author
            ("   ", 1),  # Should return Unknown Author
        ]

        for authors_text, expected_count in test_cases:
            authors = parser.parse_authors(authors_text)
            assert len(authors) == expected_count, f"Failed for: '{authors_text}'"
            assert all(isinstance(author, Author) for author in authors)
            assert all(author.name for author in authors)

    def test_parse_authors_with_emails(self, parser):
        """Test author parsing with email addresses."""
        authors_text = "John Smith (john@example.com), Jane Doe (jane.doe@university.edu)"
        authors = parser.parse_authors(authors_text)

        assert len(authors) == 2
        assert authors[0].name == "John Smith"
        assert authors[0].email == "john@example.com"
        assert authors[1].name == "Jane Doe"
        assert authors[1].email == "jane.doe@university.edu"

    def test_extract_subject_areas_from_content(self, parser):
        """Test subject area extraction from title and abstract."""
        title = "Deep Learning for Medical Image Segmentation"
        abstract = "This paper presents a convolutional neural network approach for segmenting brain MRI images."

        # Mock BeautifulSoup object with no explicit subject areas
        from bs4 import BeautifulSoup
        soup = BeautifulSoup("<html><body></body></html>", 'html.parser')

        subject_areas = parser.extract_subject_areas(soup, title, abstract)

        assert len(subject_areas) > 0
        assert any(area in ["Deep Learning", "Medical Imaging", "Computer Vision"] for area in subject_areas)

    def test_extract_external_links(self, parser, sample_miccai_html):
        """Test external link extraction from HTML."""
        from bs4 import BeautifulSoup
        soup = BeautifulSoup(sample_miccai_html, 'html.parser')
        base_url = "https://papers.miccai.org/paper/001/"

        links = parser.extract_external_links(soup, base_url)

        assert len(links) > 0

        # Check for different link types
        link_types = [link.type for link in links]
        assert 'pdf' in link_types
        assert 'github' in link_types

        # Check URL validation
        for link in links:
            assert isinstance(link, ExternalLink)
            assert link.url.startswith(('http://', 'https://'))

    def test_parse_paper_from_html_complete(self, parser, sample_miccai_html):
        """Test parsing a complete paper from HTML."""
        source_url = "https://papers.miccai.org/paper/001/"
        paper = parser.parse_paper_from_html(sample_miccai_html, source_url)

        assert paper is not None
        assert isinstance(paper, Paper)

        # Validate required fields
        assert paper.title == "Deep Learning for Medical Image Segmentation"
        assert len(paper.abstract) > 100  # Should have substantial abstract
        assert len(paper.authors) >= 1
        assert len(paper.subject_areas) >= 1
        assert paper.id.startswith("paper-") or paper.id.startswith("deep-learning")

        # Check authors
        author_names = [author.name for author in paper.authors]
        assert "John Smith" in author_names
        assert "Jane Doe" in author_names
        assert "Bob Wilson" in author_names

        # Check subject areas
        assert any(area in ["Computer Vision", "Machine Learning", "Medical Imaging"]
                  for area in paper.subject_areas)

        # Check external links
        assert len(paper.external_links) > 0

    def test_parse_paper_from_html_minimal(self, parser, minimal_miccai_html):
        """Test parsing a minimal paper from HTML."""
        source_url = "https://papers.miccai.org/paper/002/"
        paper = parser.parse_paper_from_html(minimal_miccai_html, source_url)

        assert paper is not None
        assert paper.title == "AI in Radiology: Current Applications"
        assert len(paper.abstract) > 50
        assert len(paper.authors) >= 1
        assert len(paper.subject_areas) >= 1

        # Should infer subject areas from content
        subject_text = " ".join(paper.subject_areas).lower()
        assert any(term in subject_text for term in ['medical', 'artificial', 'machine'])

    def test_parse_paper_from_html_complex_authors(self, parser, complex_authors_html):
        """Test parsing paper with complex author formatting."""
        source_url = "https://papers.miccai.org/paper/003/"
        paper = parser.parse_paper_from_html(complex_authors_html, source_url)

        assert paper is not None
        assert len(paper.authors) >= 3  # Should handle multiple delimiters

        author_names = [author.name for author in paper.authors]
        expected_names = ["Alice Smith", "Bob Jones", "Carol Wilson", "David Lee", "Emma Davis"]

        # Should extract at least most of these names
        found_names = sum(1 for name in expected_names if any(name.split()[0] in author for author in author_names))
        assert found_names >= 3

        # Check email extraction
        emails = [author.email for author in paper.authors if author.email]
        assert "emma.davis@hospital.org" in emails

    def test_parse_paper_from_invalid_html(self, parser):
        """Test handling of invalid or insufficient HTML."""
        test_cases = [
            "<html><body><h1>Only Title</h1></body></html>",  # No abstract
            "<html><body><p>Only text no structure</p></body></html>",  # No title
            "",  # Empty HTML
            "<html><body></body></html>",  # Empty body
        ]

        for html in test_cases:
            paper = parser.parse_paper_from_html(html, "https://example.com")
            # Should return None for insufficient data
            assert paper is None

    def test_parse_papers_from_json_file(self, parser, sample_raw_papers_json):
        """Test parsing papers from JSON file."""
        # Create temporary JSON file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            json.dump(sample_raw_papers_json, f, indent=2)
            temp_path = f.name

        try:
            papers = parser.parse_papers_from_json(temp_path)

            assert len(papers) >= 1  # Should parse at least one paper successfully

            for paper in papers:
                assert isinstance(paper, Paper)
                assert paper.title
                assert paper.abstract
                assert len(paper.authors) >= 1
                assert len(paper.subject_areas) >= 1

        finally:
            # Clean up
            Path(temp_path).unlink(missing_ok=True)

    def test_save_parsed_papers(self, parser):
        """Test saving parsed papers to JSON file."""
        # Create sample papers
        papers = [
            Paper(
                id="test-paper-1",
                title="Test Paper One",
                abstract="This is a test abstract for the first paper with sufficient length.",
                authors=[Author(name="Test Author One")],
                subject_areas=["Computer Vision"],
                external_links=[
                    ExternalLink(type="pdf", url="https://example.com/paper1.pdf")
                ]
            ),
            Paper(
                id="test-paper-2",
                title="Test Paper Two",
                abstract="This is another test abstract for the second paper with adequate content.",
                authors=[Author(name="Test Author Two", email="author2@example.com")],
                subject_areas=["Machine Learning", "Medical Imaging"]
            )
        ]

        # Save to temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            temp_path = f.name

        try:
            parser.save_parsed_papers(papers, temp_path)

            # Verify file was created and contains correct data
            assert Path(temp_path).exists()

            with open(temp_path, 'r', encoding='utf-8') as f:
                loaded_data = json.load(f)

            assert len(loaded_data) == 2
            assert loaded_data[0]['title'] == "Test Paper One"
            assert loaded_data[1]['title'] == "Test Paper Two"
            assert len(loaded_data[0]['authors']) == 1
            assert len(loaded_data[1]['external_links']) == 0  # Second paper has no links

        finally:
            # Clean up
            Path(temp_path).unlink(missing_ok=True)

    def test_generate_paper_id(self, parser):
        """Test paper ID generation from different inputs."""
        test_cases = [
            ("Deep Learning Paper", "https://papers.miccai.org/paper/123/", "paper-123"),
            ("AI Research", "https://example.com/random-url/", "ai-research"),
            ("", "https://papers.miccai.org/paper/456/", "paper-456"),
            ("Very Long Title That Should Be Truncated", "https://example.com/", "very-long-title-that-should"),
        ]

        for title, url, expected_pattern in test_cases:
            paper_id = parser._generate_paper_id(title, url)
            assert isinstance(paper_id, str)
            assert len(paper_id) > 0

            if "paper-" in expected_pattern:
                assert paper_id.startswith("paper-")
            else:
                # Should be based on title
                assert any(word in paper_id.lower() for word in title.lower().split() if word)

    def test_cli_interface_exists(self):
        """Test that the CLI interface is properly defined."""
        from src.lib.paper_parser import main

        # Check that it's a Click command
        assert hasattr(main, 'callback'), "Should be a Click command"
        assert hasattr(main, 'params'), "Should have parameters defined"

    @pytest.mark.integration
    def test_end_to_end_parsing_workflow(self, parser, sample_raw_papers_json):
        """Test the complete parsing workflow from raw JSON to parsed papers."""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Create input file with raw data
            input_file = Path(temp_dir) / "raw_papers.json"
            output_file = Path(temp_dir) / "parsed_papers.json"

            with open(input_file, 'w', encoding='utf-8') as f:
                json.dump(sample_raw_papers_json, f, indent=2)

            # Run parsing workflow
            papers = parser.parse_papers_from_json(str(input_file))
            parser.save_parsed_papers(papers, str(output_file))

            # Verify results
            assert len(papers) >= 1
            assert output_file.exists()

            # Verify saved data can be loaded
            with open(output_file, 'r', encoding='utf-8') as f:
                saved_data = json.load(f)

            assert len(saved_data) >= 1
            assert all('title' in paper for paper in saved_data)
            assert all('abstract' in paper for paper in saved_data)
            assert all('authors' in paper for paper in saved_data)
            assert all('subject_areas' in paper for paper in saved_data)