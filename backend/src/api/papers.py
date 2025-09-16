from fastapi import APIRouter, HTTPException, Query
from typing import List, Optional
from ..models.paper import Paper, PaperSimilarity, GraphData
from ..services.data_loader import DataLoader
from ..services.similarity import SimilarityService

router = APIRouter(prefix="/papers", tags=["papers"])

# Initialize services
data_loader = DataLoader()
similarity_service = SimilarityService(data_loader)


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
    sample_size: Optional[int] = Query(None, ge=50, le=500)
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
            sample_size=sample_size
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