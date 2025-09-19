import json
import logging
import numpy as np
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from functools import lru_cache
from ..models.paper import Paper
from ..services.data_loader import DataLoader
from ..services.similarity import SimilarityService

logger = logging.getLogger(__name__)


class TSNEService:
    """Service for handling t-SNE coordinates and similarity analysis with memoization"""

    def __init__(self, data_loader: DataLoader):
        self.data_loader = data_loader
        self.similarity_service = SimilarityService(data_loader)
        self.cache_dir = Path(__file__).parent.parent / "data" / "cache"
        self.cache_dir.mkdir(exist_ok=True)
        self.tsne_cache_file = self.cache_dir / "tsne_coordinates.json"
        self._tsne_coordinates_cache = None

    def get_tsne_coordinates(self) -> List[Dict]:
        """Get t-SNE coordinates for all papers with memoization"""
        # Check if we have cached coordinates
        if self._tsne_coordinates_cache is not None:
            return self._tsne_coordinates_cache

        # Try to load from cache file
        if self.tsne_cache_file.exists():
            try:
                with open(self.tsne_cache_file, 'r') as f:
                    cached_data = json.load(f)
                    self._tsne_coordinates_cache = cached_data
                    logger.info(f"Loaded cached t-SNE coordinates for {len(cached_data)} papers")
                    return cached_data
            except Exception as e:
                logger.error(f"Error loading cached coordinates: {e}")

        # Generate new coordinates and cache them
        coordinates = self._generate_tsne_coordinates()
        self._tsne_coordinates_cache = coordinates

        # Save to cache file
        try:
            with open(self.tsne_cache_file, 'w') as f:
                json.dump(coordinates, f, indent=2)
            logger.info(f"Cached t-SNE coordinates for {len(coordinates)} papers")
        except Exception as e:
            logger.error(f"Error saving coordinates to cache: {e}")

        return coordinates

    def _generate_tsne_coordinates(self) -> List[Dict]:
        """Generate t-SNE coordinates using the embeddings"""
        try:
            from sklearn.manifold import TSNE

            logger.info("Generating t-SNE coordinates for all papers...")

            # Load all papers
            all_papers = self.data_loader.get_all_papers()
            logger.info(f"Found {len(all_papers)} papers")

            # Load embeddings
            embeddings = []
            paper_ids = []

            for i, paper in enumerate(all_papers):
                if i % 100 == 0:
                    logger.info(f"Loading embeddings: {i}/{len(all_papers)}")

                embedding = self.data_loader.get_embedding_by_id(paper.id)
                if embedding is not None:
                    embeddings.append(embedding)
                    paper_ids.append(paper.id)

            if not embeddings:
                logger.warning("No embeddings found, using fallback coordinates")
                return self._generate_fallback_coordinates()

            logger.info(f"Loaded {len(embeddings)} embeddings, applying t-SNE...")
            embeddings_array = np.array(embeddings)

            # Apply t-SNE directly to embeddings
            tsne = TSNE(
                n_components=2,
                perplexity=min(30, len(embeddings) - 1),
                random_state=42,
                max_iter=1000,
                verbose=1
            )
            tsne_coords = tsne.fit_transform(embeddings_array)

            logger.info("t-SNE completed, creating coordinate data...")

            # Create coordinate data
            coordinates = []
            for i, paper_id in enumerate(paper_ids):
                paper = self.data_loader.get_paper_by_id(paper_id)
                if paper:
                    coordinates.append({
                        "paper_id": paper_id,
                        "tsne_x": float(tsne_coords[i, 0]),
                        "tsne_y": float(tsne_coords[i, 1]),
                        "subject_areas": paper.subject_areas,
                        "title": paper.title,
                        "authors": [author.name for author in paper.authors]
                    })

            logger.info(f"Generated coordinates for {len(coordinates)} papers")
            return coordinates

        except Exception as e:
            logger.error(f"Error generating t-SNE coordinates: {e}")
            return self._generate_fallback_coordinates()

    def _generate_fallback_coordinates(self) -> List[Dict]:
        """Generate fallback coordinates using random positioning"""
        all_papers = self.data_loader.get_all_papers()
        coordinates = []

        for i, paper in enumerate(all_papers):
            # Generate random coordinates as fallback
            coordinates.append({
                "paper_id": paper.id,
                "tsne_x": float(np.random.normal(0, 1)),
                "tsne_y": float(np.random.normal(0, 1)),
                "subject_areas": paper.subject_areas,
                "title": paper.title,
                "authors": [author.name for author in paper.authors]
            })

        return coordinates

    def get_similarity_network_data(self, paper_id: str, top_k: int = 20) -> Dict:
        """Get similarity network data for a specific paper with memoization"""
        cache_key = f"network_{paper_id}_{top_k}"
        cache_file = self.cache_dir / f"{cache_key}.json"

        # Check if we have cached network data
        if cache_file.exists():
            try:
                with open(cache_file, 'r') as f:
                    cached_data = json.load(f)
                    logger.info(f"Loaded cached network data for paper {paper_id}")
                    return cached_data
            except Exception as e:
                logger.error(f"Error loading cached network data: {e}")

        # Generate new network data
        network_data = self._generate_similarity_network_data(paper_id, top_k)

        # Save to cache
        try:
            with open(cache_file, 'w') as f:
                json.dump(network_data, f, indent=2)
            logger.info(f"Cached network data for paper {paper_id}")
        except Exception as e:
            logger.error(f"Error saving network data to cache: {e}")

        return network_data

    def _generate_similarity_network_data(self, paper_id: str, top_k: int = 20) -> Dict:
        """Generate similarity network data for a specific paper using global t-SNE coordinates"""
        try:
            logger.info(f"Generating similarity network data for paper {paper_id}...")

            # Get the target paper
            target_paper = self.data_loader.get_paper_by_id(paper_id)
            if not target_paper:
                raise ValueError(f"Paper {paper_id} not found")

            # Get similar papers
            similar_papers = self.similarity_service.find_similar_papers(paper_id, top_k)
            logger.info(f"Found {len(similar_papers)} similar papers")

            # Get global t-SNE coordinates
            global_coordinates = self.get_tsne_coordinates()
            logger.info(f"Retrieved {len(global_coordinates)} global t-SNE coordinates")

            # Create a mapping from paper_id to coordinates for quick lookup
            coords_map = {coord["paper_id"]: coord for coord in global_coordinates}

            # Get coordinates for target paper and similar papers
            subset_papers = [target_paper] + [sp.paper for sp in similar_papers]
            tsne_coordinates = []
            missing_papers = []

            for paper in subset_papers:
                if paper.id in coords_map:
                    coord = coords_map[paper.id]
                    tsne_coordinates.append({
                        "paper_id": paper.id,
                        "tsne_x": coord["tsne_x"],
                        "tsne_y": coord["tsne_y"],
                        "subject_areas": paper.subject_areas,
                        "title": paper.title,
                        "authors": [author.name for author in paper.authors]
                    })
                else:
                    missing_papers.append(paper.id)
                    logger.warning(f"Missing t-SNE coordinates for paper {paper.id}")

            if not tsne_coordinates:
                logger.warning("No t-SNE coordinates found, using fallback network data")
                return self._generate_fallback_network_data(target_paper, similar_papers)

            # Calculate cosine distances using embeddings
            subset_embeddings = []
            for paper in subset_papers:
                embedding = self.data_loader.get_embedding_by_id(paper.id)
                if embedding is not None:
                    subset_embeddings.append(embedding)
                else:
                    # Use zero embedding as fallback
                    subset_embeddings.append(np.zeros(384))

            if subset_embeddings:
                from sklearn.metrics.pairwise import cosine_distances
                embeddings_array = np.array(subset_embeddings)
                distance_matrix = cosine_distances(embeddings_array)
            else:
                # Fallback distance matrix
                n_papers = len(subset_papers)
                distance_matrix = np.ones((n_papers, n_papers))

            # Create network data
            network_data = {
                "target_paper": {
                    "id": target_paper.id,
                    "title": target_paper.title,
                    "abstract": target_paper.abstract,
                    "authors": [{"name": author.name} for author in target_paper.authors],
                    "subject_areas": target_paper.subject_areas
                },
                "similar_papers": [
                    {
                        "paper_id": sp.paper_id,
                        "similarity_score": sp.similarity_score,
                        "paper": {
                            "id": sp.paper.id,
                            "title": sp.paper.title,
                            "abstract": sp.paper.abstract,
                            "authors": [{"name": author.name} for author in sp.paper.authors],
                            "subject_areas": sp.paper.subject_areas
                        }
                    }
                    for sp in similar_papers
                ],
                "tsne_coordinates": tsne_coordinates,
                "distance_matrix": distance_matrix.tolist()
            }

            logger.info(f"Generated network data for {len(subset_papers)} papers using global t-SNE coordinates")
            return network_data

        except Exception as e:
            logger.error(f"Error generating similarity network data: {e}")
            return self._generate_fallback_network_data(target_paper, similar_papers)

    def _generate_fallback_network_data(self, target_paper, similar_papers) -> Dict:
        """Generate fallback network data"""
        return {
            "target_paper": {
                "id": target_paper.id,
                "title": target_paper.title,
                "abstract": target_paper.abstract,
                "authors": [{"name": author.name} for author in target_paper.authors],
                "subject_areas": target_paper.subject_areas
            },
            "similar_papers": [
                {
                    "paper_id": sp.paper_id,
                    "similarity_score": sp.similarity_score,
                    "paper": {
                        "id": sp.paper.id,
                        "title": sp.paper.title,
                        "abstract": sp.paper.abstract,
                        "authors": [{"name": author.name} for author in sp.paper.authors],
                        "subject_areas": sp.paper.subject_areas
                    }
                }
                for sp in similar_papers
            ],
            "tsne_coordinates": [
                {
                    "paper_id": target_paper.id,
                    "tsne_x": 0.0,
                    "tsne_y": 0.0,
                    "similarity_score": 1.0
                }
            ] + [
                {
                    "paper_id": sp.paper.id,
                    "tsne_x": float(np.random.normal(0, 1)),
                    "tsne_y": float(np.random.normal(0, 1)),
                    "similarity_score": sp.similarity_score
                }
                for sp in similar_papers
            ],
            "distance_matrix": [[0.0] * (len(similar_papers) + 1) for _ in range(len(similar_papers) + 1)]
        }
