import axios from 'axios';
import {
  Paper,
  PaperSimilarity,
  GraphData,
  DatasetStats,
  SearchParams,
  GraphParams
} from '../types/api';

const API_BASE_URL = 'http://localhost:8000/api';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
});

export class ApiService {
  // Paper operations
  static async getAllPapers(limit: number = 50, offset: number = 0): Promise<Paper[]> {
    const response = await apiClient.get<Paper[]>('/papers/', {
      params: { limit, offset }
    });
    return response.data;
  }

  static async searchPapers(params: SearchParams): Promise<Paper[]> {
    const response = await apiClient.get<Paper[]>('/papers/search', {
      params
    });
    return response.data;
  }

  static async getPaperById(paperId: string): Promise<Paper> {
    const response = await apiClient.get<Paper>(`/papers/${paperId}`);
    return response.data;
  }

  static async getSimilarPapers(paperId: string, limit: number = 10): Promise<PaperSimilarity[]> {
    const response = await apiClient.get<PaperSimilarity[]>(`/papers/${paperId}/similar`, {
      params: { limit }
    });
    return response.data;
  }

  // Graph operations
  static async getGraphData(params: GraphParams = {}): Promise<GraphData> {
    const queryParams: any = {
      similarity_threshold: params.similarity_threshold ?? 0.7,
      max_edges: params.max_edges ?? 1000,
    };

    // Add optional parameters
    if (params.sample_size !== undefined) {
      queryParams.sample_size = params.sample_size;
    }
    if (params.subject_areas && params.subject_areas.length > 0) {
      queryParams.subject_areas = params.subject_areas;
    }

    const response = await apiClient.get<GraphData>('/papers/graph/data', {
      params: queryParams
    });
    return response.data;
  }

  // Statistics
  static async getDatasetStats(): Promise<DatasetStats> {
    const response = await apiClient.get<DatasetStats>('/papers/stats/summary');
    return response.data;
  }

  static async getPaperClusters(nClusters: number = 10): Promise<Record<string, any>> {
    const response = await apiClient.get(`/papers/clusters/`, {
      params: { n_clusters: nClusters }
    });
    return response.data;
  }

  // Similarity-based clusters visualization data
  static async getClustersData(subjectAreas?: string[], sampleSize?: number): Promise<GraphData> {
    const params: any = {};
    if (subjectAreas && subjectAreas.length > 0) {
      params.subject_areas = subjectAreas;
    }
    if (sampleSize) {
      params.sample_size = sampleSize;
    }

    const response = await apiClient.get<GraphData>('/papers/clusters/data', {
      params
    });
    return response.data;
  }

  // Network visualization data for a specific paper
  static async getNetworkData(paperId: string, limit: number = 20): Promise<GraphData> {
    const response = await apiClient.get<GraphData>(`/papers/network/data`, {
      params: { paper_id: paperId, limit }
    });
    return response.data;
  }

  // t-SNE coordinates for cluster visualization
  static async getTSNECoordinates(): Promise<{ coordinates: any[] }> {
    const response = await apiClient.get('/papers/tsne-coordinates');
    return response.data;
  }

  // Similarity network data for a specific paper
  static async getSimilarityNetworkData(paperId: string, topK: number = 20): Promise<any> {
    const response = await apiClient.get(`/papers/${paperId}/similarity-network`, {
      params: { top_k: topK }
    });
    return response.data;
  }

  // Paper highlight data
  static async getPaperHighlightData(paperId: string): Promise<any> {
    const response = await apiClient.get(`/papers/${paperId}/highlight`);
    return response.data;
  }

  // Health check
  static async healthCheck(): Promise<{ status: string }> {
    const response = await apiClient.get('/health');
    return response.data;
  }
}

// Export static methods as instance methods for easier usage
export const apiService = {
  getAllPapers: ApiService.getAllPapers,
  searchPapers: ApiService.searchPapers,
  getPaperById: ApiService.getPaperById,
  getSimilarPapers: ApiService.getSimilarPapers,
  getGraphData: ApiService.getGraphData,
  getDatasetStats: ApiService.getDatasetStats,
  getPaperClusters: ApiService.getPaperClusters,
  getClustersData: ApiService.getClustersData,
  getNetworkData: ApiService.getNetworkData,
  getTSNECoordinates: ApiService.getTSNECoordinates,
  getSimilarityNetworkData: ApiService.getSimilarityNetworkData,
  getPaperHighlightData: ApiService.getPaperHighlightData,
  healthCheck: ApiService.healthCheck,
};

export default ApiService;
