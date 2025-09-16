import json
import numpy as np
from pathlib import Path
from typing import Dict, List, Optional
from ..models.paper import Paper


class DataLoader:
    def __init__(self, data_dir: str = "src/data"):
        self.data_dir = Path(data_dir)
        self.papers_dir = self.data_dir / "papers_by_id"
        self.embeddings_dir = self.data_dir / "embeddings_by_id"
        self._papers_cache: Dict[str, Paper] = {}
        self._embeddings_cache: Dict[str, np.ndarray] = {}
        self._paper_index: Optional[Dict] = None

    def load_paper_index(self) -> Dict:
        """Load the main paper index with all paper IDs and metadata"""
        if self._paper_index is None:
            index_path = self.papers_dir / "index.json"
            with open(index_path, 'r', encoding='utf-8') as f:
                self._paper_index = json.load(f)
        return self._paper_index or {}

    def get_paper_by_id(self, paper_id: str) -> Optional[Paper]:
        """Load a single paper by ID"""
        if paper_id in self._papers_cache:
            return self._papers_cache[paper_id]

        paper_path = self.papers_dir / f"{paper_id}.json"
        if not paper_path.exists():
            return None

        with open(paper_path, 'r', encoding='utf-8') as f:
            paper_data = json.load(f)

        paper = Paper(**paper_data)
        self._papers_cache[paper_id] = paper
        return paper

    def get_all_papers(self) -> List[Paper]:
        """Load all papers"""
        index = self.load_paper_index()
        papers = []

        # The index contains paper objects, not just IDs
        for paper_data in index.get("papers", []):
            if isinstance(paper_data, dict) and "id" in paper_data:
                paper = self.get_paper_by_id(paper_data["id"])
                if paper:
                    papers.append(paper)

        return papers

    def get_embedding_by_id(self, paper_id: str) -> Optional[np.ndarray]:
        """Load paper embedding by ID"""
        if paper_id in self._embeddings_cache:
            return self._embeddings_cache[paper_id]

        embedding_path = self.embeddings_dir / f"{paper_id}_embedding.npz"
        if not embedding_path.exists():
            return None

        with np.load(embedding_path) as data:
            embedding = data['embedding']

        self._embeddings_cache[paper_id] = embedding
        return embedding

    def get_all_embeddings(self) -> Dict[str, np.ndarray]:
        """Load all embeddings"""
        index = self.load_paper_index()
        embeddings = {}

        # The index contains paper objects, not just IDs
        for paper_data in index.get("papers", []):
            if isinstance(paper_data, dict) and "id" in paper_data:
                paper_id = paper_data["id"]
                embedding = self.get_embedding_by_id(paper_id)
                if embedding is not None:
                    embeddings[paper_id] = embedding

        return embeddings

    def search_papers(self, query: str, limit: int = 20) -> List[Paper]:
        """Simple text search in paper titles and abstracts"""
        all_papers = self.get_all_papers()
        query_lower = query.lower()

        matching_papers = []
        for paper in all_papers:
            if (query_lower in paper.title.lower() or
                query_lower in paper.abstract.lower() or
                any(query_lower in area.lower() for area in paper.subject_areas) or
                any(query_lower in author.name.lower() for author in paper.authors)):
                matching_papers.append(paper)

        return matching_papers[:limit]
