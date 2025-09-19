export interface Author {
  name: string;
  affiliation?: string;
  email?: string;
}

export interface ExternalLink {
  type: string;
  url: string;
  description?: string;
}

export interface Paper {
  id: string;
  title: string;
  abstract: string;
  authors: Author[];
  subject_areas: string[];
  external_links: ExternalLink[];
  publication_date?: string;
  raw_data_source?: string;
}

export interface PaperSimilarity {
  paper_id: string;
  similarity_score: number;
  paper: Paper;
}

export interface GraphNode {
  id: string;
  title: string;
  authors: string[];
  subject_areas: string[];
  x?: number;
  y?: number;
  cluster?: number;
}

export interface GraphEdge {
  source: string;
  target: string;
  similarity: number;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
  clusters?: Record<string, any>;
}

export interface DatasetStats {
  total_papers: number;
  total_authors: number;
  subject_areas: Record<string, number>;
  dataset_info: Record<string, any>;
}

export interface SearchParams {
  q: string;
  limit?: number;
}

export interface GraphParams {
  similarity_threshold?: number;
  max_edges?: number;
  sample_size?: number;
  subject_areas?: string[];
}

export type ViewMode = 'clusters' | 'similarity';
