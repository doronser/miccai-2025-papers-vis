from fastapi import APIRouter, HTTPException, Query
from typing import List, Optional, Dict, Any
import logging
from ..models.paper import Paper, PaperSimilarity, GraphData
from ..services.data_loader import DataLoader
from ..services.similarity import SimilarityService
from ..services.tsne_service import TSNEService

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/papers", tags=["papers"])

# Initialize services
data_loader = DataLoader()
similarity_service = SimilarityService(data_loader)
tsne_service = TSNEService(data_loader)


@router.get("/", response_model=List[Paper])
async def get_papers(
    limit: int = Query(50, ge=1, le=1000),
    offset: int = Query(0, ge=0)
):
    """Get all papers with pagination"""
    all_papers = data_loader.get_all_papers()
    total = len(all_papers)

    if offset >= total:
        return []

    return all_papers[offset:offset + limit]


@router.get("/search", response_model=List[Paper])
async def search_papers(
    q: str = Query(..., min_length=1),
    limit: int = Query(20, ge=1, le=100)
):
    """Search papers by title, abstract, authors, or subject areas"""
    results = data_loader.search_papers(q, limit)
    return results


@router.get("/tsne-coordinates")
async def get_tsne_coordinates():
    """Get t-SNE coordinates for all papers"""
    logger.info("t-SNE coordinates endpoint accessed")
    try:
        coordinates = tsne_service.get_tsne_coordinates()
        logger.info(f"Returning {len(coordinates)} t-SNE coordinates")
        return {"coordinates": coordinates}
    except Exception as e:
        logger.error(f"Error getting t-SNE coordinates: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error getting t-SNE coordinates: {str(e)}")


@router.get("/{paper_id}", response_model=Paper)
async def get_paper(paper_id: str):
    """Get a specific paper by ID"""
    paper = data_loader.get_paper_by_id(paper_id)
    if not paper:
        raise HTTPException(status_code=404, detail="Paper not found")
    return paper


@router.get("/{paper_id}/similar", response_model=List[PaperSimilarity])
async def get_similar_papers(
    paper_id: str,
    limit: int = Query(10, ge=1, le=50)
):
    """Get papers similar to the specified paper"""
    paper = data_loader.get_paper_by_id(paper_id)
    if not paper:
        raise HTTPException(status_code=404, detail="Paper not found")

    similar_papers = similarity_service.find_similar_papers(paper_id, limit)
    return similar_papers


@router.get("/graph/data", response_model=GraphData)
async def get_graph_data(
    similarity_threshold: float = Query(0.7, ge=0.0, le=1.0),
    max_edges: int = Query(1000, ge=100, le=5000),
    sample_size: Optional[int] = Query(None, ge=50, le=500),
    subject_areas: Optional[List[str]] = Query(None)
):
    """Get graph data for visualization with papers as nodes and similarities as edges"""
    try:
        # For performance, we might want to sample papers for large datasets
        if sample_size:
            # TODO: Implement sampling logic if needed for performance
            pass

        graph_data = similarity_service.generate_graph_data(
            similarity_threshold=similarity_threshold,
            max_edges=max_edges,
            sample_size=sample_size,
            subject_areas=subject_areas
        )
        return graph_data
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating graph data: {str(e)}")


@router.get("/stats/summary")
async def get_dataset_stats():
    """Get dataset statistics"""
    try:
        index = data_loader.load_paper_index()
        all_papers = data_loader.get_all_papers()

        # Calculate author statistics
        all_authors = set()
        subject_area_counts = {}

        for paper in all_papers:
            for author in paper.authors:
                all_authors.add(author.name)

            for area in paper.subject_areas:
                subject_area_counts[area] = subject_area_counts.get(area, 0) + 1

        return {
            "total_papers": len(all_papers),
            "total_authors": len(all_authors),
            "subject_areas": dict(sorted(subject_area_counts.items(), key=lambda x: x[1], reverse=True)),
            "dataset_info": index.get("dataset_info", {})
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error getting dataset stats: {str(e)}")


@router.get("/clusters/")
async def get_paper_clusters(n_clusters: int = Query(10, ge=2, le=20)):
    """Get paper clusters based on similarity"""
    try:
        clusters = similarity_service.get_paper_clusters(n_clusters)

        # Add paper details to each cluster
        detailed_clusters = {}
        for cluster_id, paper_ids in clusters.items():
            papers = []
            for pid in paper_ids:
                paper = data_loader.get_paper_by_id(pid)
                if paper:
                    papers.append({
                        "id": paper.id,
                        "title": paper.title,
                        "authors": [a.name for a in paper.authors]
                    })

            detailed_clusters[cluster_id] = {
                "papers": papers,
                "size": len(papers)
            }

        return detailed_clusters
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error getting clusters: {str(e)}")


@router.get("/clusters/data")
async def get_clusters_data(
    subject_areas: Optional[List[str]] = Query(None),
    sample_size: Optional[int] = Query(None, ge=50, le=1000)
):
    """Get similarity-based clustering visualization data for all papers"""
    try:
        clusters_data = similarity_service.generate_similarity_clusters_data(
            subject_areas=subject_areas,
            sample_size=sample_size
        )
        return clusters_data
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating clusters data: {str(e)}")


@router.get("/network/data")
async def get_network_data(
    paper_id: str,
    limit: int = Query(20, ge=5, le=50)
):
    """Get network data for a specific paper showing top similar papers"""
    try:
        network_data = similarity_service.generate_network_data(paper_id, limit)
        return network_data
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating network data: {str(e)}")


@router.get("/{paper_id}/similarity-network")
async def get_similarity_network(
    paper_id: str,
    top_k: int = Query(20, ge=5, le=50)
):
    """Get similarity network data for a specific paper with t-SNE coordinates"""
    try:
        # Check if paper exists
        paper = data_loader.get_paper_by_id(paper_id)
        if not paper:
            raise HTTPException(status_code=404, detail="Paper not found")

        network_data = tsne_service.get_similarity_network_data(paper_id, top_k)
        return network_data
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating similarity network: {str(e)}")


@router.get("/{paper_id}/highlight")
async def highlight_paper(paper_id: str):
    """Get paper data for highlighting in cluster view"""
    try:
        paper = data_loader.get_paper_by_id(paper_id)
        if not paper:
            raise HTTPException(status_code=404, detail="Paper not found")

        return {
            "paper_id": paper.id,
            "title": paper.title,
            "subject_areas": paper.subject_areas,
            "authors": [author.name for author in paper.authors]
        }
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error getting paper highlight data: {str(e)}")
