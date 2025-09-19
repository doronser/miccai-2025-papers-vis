import axios from 'axios';
import ApiService from '../services/api';
import { Paper, GraphData, DatasetStats } from '../types/api';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

const mockApiClient = {
  get: jest.fn(),
  post: jest.fn(),
  put: jest.fn(),
  delete: jest.fn(),
};

// Mock axios.create
mockedAxios.create = jest.fn(() => mockApiClient as any);

describe('ApiService', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('getAllPapers', () => {
    test('fetches papers with default pagination', async () => {
      const mockPapers: Paper[] = [
        {
          id: 'paper-1',
          title: 'Test Paper',
          abstract: 'Test abstract',
          authors: [{ name: 'Test Author' }],
          subject_areas: ['Computer Vision'],
          external_links: [],
          publication_date: '2025-01-01'
        }
      ];

      mockApiClient.get.mockResolvedValue({ data: mockPapers });

      const result = await ApiService.getAllPapers();

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/', {
        params: { limit: 50, offset: 0 }
      });
      expect(result).toEqual(mockPapers);
    });

    test('fetches papers with custom pagination', async () => {
      const mockPapers: Paper[] = [];
      mockApiClient.get.mockResolvedValue({ data: mockPapers });

      await ApiService.getAllPapers(20, 10);

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/', {
        params: { limit: 20, offset: 10 }
      });
    });
  });

  describe('searchPapers', () => {
    test('searches papers with query', async () => {
      const mockResults: Paper[] = [];
      mockApiClient.get.mockResolvedValue({ data: mockResults });

      const result = await ApiService.searchPapers({ q: 'medical imaging' });

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/search', {
        params: { q: 'medical imaging' }
      });
      expect(result).toEqual(mockResults);
    });

    test('searches papers with query and limit', async () => {
      const mockResults: Paper[] = [];
      mockApiClient.get.mockResolvedValue({ data: mockResults });

      await ApiService.searchPapers({ q: 'medical imaging', limit: 10 });

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/search', {
        params: { q: 'medical imaging', limit: 10 }
      });
    });
  });

  describe('getPaperById', () => {
    test('fetches paper by ID', async () => {
      const mockPaper: Paper = {
        id: 'paper-1',
        title: 'Test Paper',
        abstract: 'Test abstract',
        authors: [{ name: 'Test Author' }],
        subject_areas: ['Computer Vision'],
        external_links: [],
        publication_date: '2025-01-01'
      };

      mockApiClient.get.mockResolvedValue({ data: mockPaper });

      const result = await ApiService.getPaperById('paper-1');

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/paper-1');
      expect(result).toEqual(mockPaper);
    });
  });

  describe('getSimilarPapers', () => {
    test('fetches similar papers with default limit', async () => {
      const mockSimilar = [
        {
          paper_id: 'paper-2',
          similarity_score: 0.8,
          paper: {
            id: 'paper-2',
            title: 'Similar Paper',
            abstract: 'Similar abstract',
            authors: [{ name: 'Similar Author' }],
            subject_areas: ['Computer Vision'],
            external_links: [],
          }
        }
      ];

      mockApiClient.get.mockResolvedValue({ data: mockSimilar });

      const result = await ApiService.getSimilarPapers('paper-1');

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/paper-1/similar', {
        params: { limit: 10 }
      });
      expect(result).toEqual(mockSimilar);
    });

    test('fetches similar papers with custom limit', async () => {
      mockApiClient.get.mockResolvedValue({ data: [] });

      await ApiService.getSimilarPapers('paper-1', 5);

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/paper-1/similar', {
        params: { limit: 5 }
      });
    });
  });

  describe('getGraphData', () => {
    test('fetches graph data with default parameters', async () => {
      const mockGraphData: GraphData = {
        nodes: [
          {
            id: 'paper-1',
            title: 'Test Paper',
            authors: ['Test Author'],
            subject_areas: ['Computer Vision']
          }
        ],
        edges: [],
        clusters: {}
      };

      mockApiClient.get.mockResolvedValue({ data: mockGraphData });

      const result = await ApiService.getGraphData();

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/graph/data', {
        params: {
          similarity_threshold: 0.7,
          max_edges: 1000
        }
      });
      expect(result).toEqual(mockGraphData);
    });

    test('fetches graph data with custom parameters', async () => {
      const mockGraphData: GraphData = { nodes: [], edges: [], clusters: {} };
      mockApiClient.get.mockResolvedValue({ data: mockGraphData });

      await ApiService.getGraphData({
        similarity_threshold: 0.8,
        max_edges: 500,
        sample_size: 100
      });

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/graph/data', {
        params: {
          similarity_threshold: 0.8,
          max_edges: 500,
          sample_size: 100
        }
      });
    });
  });

  describe('getDatasetStats', () => {
    test('fetches dataset statistics', async () => {
      const mockStats: DatasetStats = {
        total_papers: 1007,
        total_authors: 4903,
        subject_areas: {
          'Computer Vision': 150,
          'Medical Imaging': 200
        },
        dataset_info: {}
      };

      mockApiClient.get.mockResolvedValue({ data: mockStats });

      const result = await ApiService.getDatasetStats();

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/stats/summary');
      expect(result).toEqual(mockStats);
    });
  });

  describe('getPaperClusters', () => {
    test('fetches paper clusters with default parameters', async () => {
      const mockClusters = {
        '0': { papers: [{ id: 'paper-1', title: 'Test' }], size: 1 },
        '1': { papers: [{ id: 'paper-2', title: 'Test 2' }], size: 1 }
      };

      mockApiClient.get.mockResolvedValue({ data: mockClusters });

      const result = await ApiService.getPaperClusters();

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/clusters/', {
        params: { n_clusters: 10 }
      });
      expect(result).toEqual(mockClusters);
    });

    test('fetches paper clusters with custom parameters', async () => {
      mockApiClient.get.mockResolvedValue({ data: {} });

      await ApiService.getPaperClusters(5);

      expect(mockApiClient.get).toHaveBeenCalledWith('/papers/clusters/', {
        params: { n_clusters: 5 }
      });
    });
  });

  describe('error handling', () => {
    test('handles API errors', async () => {
      mockApiClient.get.mockRejectedValue(new Error('Network error'));

      await expect(ApiService.getAllPapers()).rejects.toThrow('Network error');
    });
  });
});