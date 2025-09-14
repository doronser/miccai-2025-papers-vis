#!/usr/bin/env python3
"""
Paper Data Parser Library

Parses raw paper HTML/data from MICCAI and converts it into structured Paper entities.
Provides both programmatic API and CLI interface.
"""

import json
import logging
import re
from pathlib import Path
from typing import Dict, List, Optional, Union
from urllib.parse import urljoin, urlparse

import click
from bs4 import BeautifulSoup
from pydantic import BaseModel, Field, field_validator


# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class Author(BaseModel):
    """Author entity with validation."""
    name: str = Field(..., min_length=1, max_length=200)
    affiliation: Optional[str] = Field(default=None, max_length=300)
    email: Optional[str] = None

    @field_validator('email')
    @classmethod
    def validate_email(cls, v: Optional[str]) -> Optional[str]:
        if v is None:
            return v
        # Basic email validation
        if '@' not in v or '.' not in v.split('@')[-1]:
            return None  # Invalid email, return None instead of raising
        return v.strip().lower()

    @field_validator('name')
    @classmethod
    def validate_name(cls, v: str) -> str:
        return v.strip()


class ExternalLink(BaseModel):
    """External link entity with validation."""
    type: str = Field(..., pattern=r'^(github|dataset|website|pdf)$')
    url: str = Field(..., min_length=1)
    description: Optional[str] = Field(None, max_length=200)

    @field_validator('url')
    @classmethod
    def validate_url(cls, v: str) -> str:
        v = v.strip()
        if not v.startswith(('http://', 'https://', 'ftp://')):
            raise ValueError('URL must start with http://, https://, or ftp://')
        return v


class Paper(BaseModel):
    """Main paper entity with full validation."""
    id: str = Field(..., min_length=1)
    title: str = Field(..., min_length=1, max_length=500)
    abstract: str = Field(..., min_length=10, max_length=5000)
    authors: List[Author] = Field(default_factory=list, min_length=1)
    subject_areas: List[str] = Field(default_factory=list, min_length=1)
    external_links: List[ExternalLink] = Field(default_factory=list)
    publication_date: str = Field(default="2025-01-15")  # MICCAI 2025 default
    raw_data_source: Optional[str] = None

    @field_validator('id')
    @classmethod
    def validate_id(cls, v: str) -> str:
        # Generate clean ID from title or URL
        clean_id = re.sub(r'[^a-zA-Z0-9-_]', '-', v.strip().lower())
        clean_id = re.sub(r'-+', '-', clean_id).strip('-')
        return clean_id or f"paper-{hash(v) % 10000:04d}"

    @field_validator('title')
    @classmethod
    def validate_title(cls, v: str) -> str:
        return v.strip()

    @field_validator('abstract')
    @classmethod
    def validate_abstract(cls, v: str) -> str:
        return v.strip()

    @field_validator('subject_areas')
    @classmethod
    def validate_subject_areas(cls, v: List[str]) -> List[str]:
        # Clean and deduplicate subject areas
        cleaned = [area.strip() for area in v if area.strip()]
        return list(dict.fromkeys(cleaned))  # Remove duplicates while preserving order


class PaperParser:
    """Parser for converting raw MICCAI paper data into structured Paper entities."""

    def __init__(self):
        self.default_subject_areas = [
            "Computer Vision", "Machine Learning", "Medical Imaging",
            "Deep Learning", "Image Processing", "Artificial Intelligence"
        ]

    def parse_authors(self, authors_text: str) -> List[Author]:
        """Parse author string into Author objects."""
        if not authors_text or not authors_text.strip():
            return [Author(name="Unknown Author")]

        # Split by common delimiters
        author_patterns = [
            r',\s*',          # comma separated
            r';\s*',          # semicolon separated
            r'\s+and\s+',     # "and" separated
            r'\s*\|\s*',      # pipe separated
        ]

        authors_list = [authors_text]
        for pattern in author_patterns:
            new_list = []
            for author_text in authors_list:
                new_list.extend(re.split(pattern, author_text, flags=re.IGNORECASE))
            authors_list = new_list

        authors = []
        for author_text in authors_list:
            author_text = author_text.strip()
            if not author_text:
                continue

            # Extract email if present
            email_match = re.search(r'\b[\w.-]+@[\w.-]+\.\w+\b', author_text)
            email = email_match.group() if email_match else None

            # Extract name (remove email and common patterns)
            name = re.sub(r'\b[\w.-]+@[\w.-]+\.\w+\b', '', author_text).strip()
            name = re.sub(r'\s*\([^)]*\)\s*', '', name)  # Remove parentheses
            name = re.sub(r'\s+', ' ', name).strip()

            if name:
                authors.append(Author(name=name, email=email))

        return authors if authors else [Author(name="Unknown Author")]

    def extract_subject_areas(self, soup: BeautifulSoup, title: str, abstract: str) -> List[str]:
        """Extract subject areas from HTML or infer from content."""
        subject_areas = []

        # Try to find explicit subject areas in HTML
        subject_selectors = [
            '.subject-areas', '.keywords', '.tags', '.categories',
            '#subject-areas', '#keywords', '#tags'
        ]

        for selector in subject_selectors:
            elem = soup.select_one(selector)
            if elem:
                text = elem.get_text()
                # Split by common delimiters
                areas = re.split(r'[,;|]', text)
                subject_areas.extend([area.strip() for area in areas if area.strip()])

        # If no explicit areas found, infer from title and abstract
        if not subject_areas:
            content = f"{title} {abstract}".lower()

            # Medical imaging keywords
            if any(term in content for term in ['medical', 'clinical', 'radiology', 'mri', 'ct', 'ultrasound']):
                subject_areas.append("Medical Imaging")

            # AI/ML keywords
            if any(term in content for term in ['deep learning', 'neural network', 'cnn', 'transformer']):
                subject_areas.append("Deep Learning")
            elif any(term in content for term in ['machine learning', 'classification', 'regression']):
                subject_areas.append("Machine Learning")

            # Computer vision keywords
            if any(term in content for term in ['segmentation', 'detection', 'recognition', 'computer vision']):
                subject_areas.append("Computer Vision")

            # Image processing keywords
            if any(term in content for term in ['image processing', 'filtering', 'enhancement']):
                subject_areas.append("Image Processing")

        # Fallback to default if still empty
        if not subject_areas:
            subject_areas = ["Medical Imaging"]

        # Clean and deduplicate
        cleaned_areas = []
        for area in subject_areas:
            area = area.strip().title()
            if area and area not in cleaned_areas:
                cleaned_areas.append(area)

        return cleaned_areas

    def extract_external_links(self, soup: BeautifulSoup, base_url: str) -> List[ExternalLink]:
        """Extract external links from HTML."""
        links = []

        for link in soup.find_all('a', href=True):
            href = str(link['href'])  # type: ignore
            text = link.get_text().strip().lower()

            if not href or not isinstance(href, str):
                continue

            # Make URL absolute
            full_url = urljoin(base_url, href)

            # Categorize links
            if any(domain in href for domain in ['github.com', 'gitlab.com', 'bitbucket.org']):
                links.append(ExternalLink(
                    type='github',
                    url=full_url,
                    description=link.get_text().strip() or 'Source code'
                ))
            elif href.endswith('.pdf'):
                links.append(ExternalLink(
                    type='pdf',
                    url=full_url,
                    description=link.get_text().strip() or 'PDF download'
                ))
            elif any(term in text for term in ['dataset', 'data']):
                links.append(ExternalLink(
                    type='dataset',
                    url=full_url,
                    description=link.get_text().strip() or 'Dataset'
                ))
            elif href.startswith(('http://', 'https://')) and not any(
                domain in href for domain in ['miccai.org', 'localhost', '127.0.0.1']
            ):
                links.append(ExternalLink(
                    type='website',
                    url=full_url,
                    description=link.get_text().strip() or 'External website'
                ))

        # Remove duplicates
        seen_urls = set()
        unique_links = []
        for link in links:
            if link.url not in seen_urls:
                seen_urls.add(link.url)
                unique_links.append(link)

        return unique_links

    def parse_paper_from_html(self, html_content: str, source_url: str) -> Optional[Paper]:
        """Parse a paper from raw HTML content."""
        try:
            soup = BeautifulSoup(html_content, 'html.parser')

            # Extract title
            title = None
            title_selectors = ['h1', '.title', '#title', 'title']
            for selector in title_selectors:
                elem = soup.select_one(selector)
                if elem:
                    title = elem.get_text().strip()
                    if title and title.lower() != 'miccai 2025':  # Skip generic titles
                        break

            if not title:
                logger.warning(f"No title found for {source_url}")
                return None

            # Extract abstract
            abstract = None
            abstract_selectors = ['.abstract', '#abstract', '.summary', '.paper-abstract']
            for selector in abstract_selectors:
                elem = soup.select_one(selector)
                if elem:
                    abstract = elem.get_text().strip()
                    if len(abstract) > 50:  # Ensure it's substantial
                        break

            if not abstract:
                # Fallback: look for longer paragraphs
                paragraphs = soup.find_all('p')
                for p in paragraphs:
                    text = p.get_text().strip()
                    if len(text) > 100:
                        abstract = text
                        break

            if not abstract:
                logger.warning(f"No abstract found for {source_url}")
                return None

            # Extract authors
            authors_text = ""
            author_selectors = ['.authors', '.author', '#authors', '.paper-authors']
            for selector in author_selectors:
                elem = soup.select_one(selector)
                if elem:
                    authors_text = elem.get_text().strip()
                    break

            authors = self.parse_authors(authors_text)

            # Extract subject areas
            subject_areas = self.extract_subject_areas(soup, title, abstract)

            # Extract external links
            external_links = self.extract_external_links(soup, source_url)

            # Generate paper ID
            paper_id = self._generate_paper_id(title, source_url)

            # Create Paper object
            paper = Paper(
                id=paper_id,
                title=title,
                abstract=abstract,
                authors=authors,
                subject_areas=subject_areas,
                external_links=external_links,
                raw_data_source=source_url
            )

            return paper

        except Exception as e:
            logger.error(f"Error parsing paper from {source_url}: {e}")
            return None

    def _generate_paper_id(self, title: str, url: str) -> str:
        """Generate a unique paper ID."""
        # Try to extract ID from URL first
        url_parts = urlparse(url).path.split('/')
        for part in reversed(url_parts):
            if part and re.match(r'^[a-zA-Z0-9-_]+$', part):
                return f"paper-{part}"

        # Generate from title
        clean_title = re.sub(r'[^a-zA-Z0-9\s]', '', title.lower())
        words = clean_title.split()[:5]  # Take first 5 words
        paper_id = '-'.join(words)

        return paper_id or f"paper-{abs(hash(url)) % 10000:04d}"

    def parse_papers_from_json(self, json_file_path: str) -> List[Paper]:
        """Parse papers from raw JSON data file."""
        papers = []

        try:
            with open(json_file_path, 'r', encoding='utf-8') as f:
                raw_papers = json.load(f)

            for raw_paper in raw_papers:
                if not isinstance(raw_paper, dict):
                    continue

                html_content = raw_paper.get('html_content')
                source_url = raw_paper.get('url') or raw_paper.get('source_url')

                if html_content and source_url:
                    paper = self.parse_paper_from_html(html_content, source_url)
                    if paper:
                        papers.append(paper)

        except Exception as e:
            logger.error(f"Error parsing papers from {json_file_path}: {e}")

        logger.info(f"Successfully parsed {len(papers)} papers from {json_file_path}")
        return papers

    def save_parsed_papers(self, papers: List[Paper], output_path: str) -> None:
        """Save parsed papers to JSON file."""
        output_file = Path(output_path)
        output_file.parent.mkdir(parents=True, exist_ok=True)

        # Convert to dict for JSON serialization
        papers_dict = [paper.model_dump() for paper in papers]

        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(papers_dict, f, indent=2, ensure_ascii=False)

        logger.info(f"Saved {len(papers)} parsed papers to {output_file}")


# CLI Interface
@click.command()
@click.option('--input', '-i', required=True,
              help='Input JSON file with raw paper data')
@click.option('--output', '-o', default='data/parsed_papers.json',
              help='Output file for parsed papers')
@click.option('--verbose', '-v', is_flag=True, help='Enable verbose logging')
def main(input: str, output: str, verbose: bool):
    """
    Paper Parser CLI

    Parses raw paper data from JSON file and converts to structured format.

    Example:
        python -m src.lib.paper_parser -i data/raw_papers.json -o data/parsed_papers.json
    """
    if verbose:
        logging.getLogger().setLevel(logging.DEBUG)

    parser = PaperParser()

    try:
        papers = parser.parse_papers_from_json(input)
        if papers:
            parser.save_parsed_papers(papers, output)
            click.echo(f"✅ Successfully parsed {len(papers)} papers")
            click.echo(f"📄 Data saved to: {output}")
        else:
            click.echo("❌ No papers were successfully parsed")

    except Exception as e:
        click.echo(f"❌ Error: {e}")
        raise


if __name__ == "__main__":
    main()
