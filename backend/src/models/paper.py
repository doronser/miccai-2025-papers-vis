from typing import List, Optional
from pydantic import BaseModel


class Author(BaseModel):
    name: str
    affiliation: Optional[str] = None
    email: Optional[str] = None


class ExternalLink(BaseModel):
    type: str
    url: str
    description: Optional[str] = None


class Paper(BaseModel):
    id: str
    title: str
    abstract: str
    authors: List[Author]
    subject_areas: List[str] = []
    external_links: List[ExternalLink] = []
    publication_date: Optional[str] = None
    raw_data_source: Optional[str] = None


class PaperSimilarity(BaseModel):
    paper_id: str
    similarity_score: float
    paper: Paper


class GraphNode(BaseModel):
    id: str
    title: str
    authors: List[str]
    subject_areas: List[str]
    x: Optional[float] = None
    y: Optional[float] = None
    cluster: Optional[int] = None


class GraphEdge(BaseModel):
    source: str
    target: str
    similarity: float


class GraphData(BaseModel):
    nodes: List[GraphNode]
    edges: List[GraphEdge]
    clusters: Optional[dict] = None