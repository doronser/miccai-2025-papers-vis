import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import App from '../App';
import ApiService from '../services/api';

// Mock the API service
jest.mock('../services/api');
const mockApiService = ApiService as jest.Mocked<typeof ApiService>;

// Mock D3 to avoid DOM manipulation in tests
jest.mock('d3', () => ({
  select: jest.fn(() => ({
    selectAll: jest.fn(() => ({ remove: jest.fn() })),
    append: jest.fn(() => ({
      selectAll: jest.fn(() => ({
        data: jest.fn(() => ({
          enter: jest.fn(() => ({
            append: jest.fn(() => ({
              attr: jest.fn().mockReturnThis(),
              style: jest.fn().mockReturnThis(),
              text: jest.fn().mockReturnThis(),
              on: jest.fn().mockReturnThis(),
              call: jest.fn().mockReturnThis(),
            }))
          }))
        }))
      }))
    })),
    attr: jest.fn().mockReturnThis(),
    call: jest.fn().mockReturnThis(),
  })),
  zoom: jest.fn(() => ({
    scaleExtent: jest.fn().mockReturnThis(),
    on: jest.fn().mockReturnThis(),
  })),
  forceSimulation: jest.fn(() => ({
    force: jest.fn().mockReturnThis(),
    on: jest.fn().mockReturnThis(),
    stop: jest.fn(),
  })),
  forceLink: jest.fn(() => ({
    id: jest.fn().mockReturnThis(),
    distance: jest.fn().mockReturnThis(),
    strength: jest.fn().mockReturnThis(),
  })),
  forceManyBody: jest.fn(() => ({
    strength: jest.fn().mockReturnThis(),
  })),
  forceCenter: jest.fn(),
  forceCollide: jest.fn(() => ({
    radius: jest.fn().mockReturnThis(),
  })),
  scaleOrdinal: jest.fn(),
  schemeCategory10: [],
  drag: jest.fn(() => ({
    on: jest.fn().mockReturnThis(),
  })),
}));

const mockGraphData = {
  nodes: [
    {
      id: 'paper-1',
      title: 'Test Paper 1',
      authors: ['Author 1', 'Author 2'],
      subject_areas: ['Computer Vision'],
      cluster: 0
    },
    {
      id: 'paper-2',
      title: 'Test Paper 2',
      authors: ['Author 3'],
      subject_areas: ['Medical Imaging'],
      cluster: 1
    }
  ],
  edges: [
    {
      source: 'paper-1',
      target: 'paper-2',
      similarity: 0.8
    }
  ],
  clusters: {
    '0': { id: 0, papers: ['paper-1'] },
    '1': { id: 1, papers: ['paper-2'] }
  }
};

const mockDatasetStats = {
  total_papers: 1007,
  total_authors: 4903,
  subject_areas: {
    'Computer Vision': 150,
    'Medical Imaging': 200,
    'Machine Learning': 180
  },
  dataset_info: {}
};

describe('App Component', () => {
  beforeEach(() => {
    jest.clearAllMocks();

    mockApiService.getGraphData.mockResolvedValue(mockGraphData);
    mockApiService.getDatasetStats.mockResolvedValue(mockDatasetStats);
    mockApiService.searchPapers.mockResolvedValue([]);
  });

  test('renders main title', async () => {
    render(<App />);

    expect(screen.getByText('MICCAI 2025 Papers Visualization')).toBeInTheDocument();
  });

  test('loads and displays dataset stats', async () => {
    render(<App />);

    await waitFor(() => {
      expect(screen.getByText(/1,007/)).toBeInTheDocument(); // total papers
      expect(screen.getByText(/4,903/)).toBeInTheDocument(); // total authors
    });
  });

  test('shows loading state initially', () => {
    render(<App />);

    expect(screen.getByText(/Loading graph visualization.../)).toBeInTheDocument();
  });

  test('calls API services on mount', async () => {
    render(<App />);

    await waitFor(() => {
      expect(mockApiService.getGraphData).toHaveBeenCalled();
      expect(mockApiService.getDatasetStats).toHaveBeenCalled();
    });
  });

  test('handles API errors gracefully', async () => {
    mockApiService.getGraphData.mockRejectedValue(new Error('API Error'));

    render(<App />);

    await waitFor(() => {
      expect(screen.getByText(/Failed to load graph data/)).toBeInTheDocument();
    });
  });

  test('displays search panel', () => {
    render(<App />);

    expect(screen.getByPlaceholderText(/Search papers by title/)).toBeInTheDocument();
  });

  test('shows graph visualization when data loaded', async () => {
    render(<App />);

    await waitFor(() => {
      expect(screen.queryByText(/Loading graph visualization.../)).not.toBeInTheDocument();
    });
  });
});