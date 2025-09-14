# Data Model: MICCAI 2025 Papers Visualization

## Core Entities

### Paper
Primary entity representing a MICCAI 2025 accepted paper.

**Fields**:
- `id`: string (unique identifier, derived from title or URL)
- `title`: string (required, max 500 chars)
- `abstract`: string (required, max 5000 chars)
- `authors`: Author[] (required, at least 1)
- `subject_areas`: string[] (required, at least 1)
- `external_links`: ExternalLink[] (optional)
- `publication_date`: string (ISO date, required)
- `embedding_vector`: float[] (384 dimensions for MiniLM model)

**Validation Rules**:
- Title must be unique across all papers
- Abstract cannot be empty
- At least one author required
- Subject areas must be from predefined conference categories

**State Transitions**:
- Raw → Processed (after embedding generation)
- Processed → Indexed (after search index creation)

### Author
Represents paper authors with affiliation information.

**Fields**:
- `name`: string (required, max 200 chars)
- `affiliation`: string (optional, max 300 chars)
- `email`: string (optional, validated email format)

**Validation Rules**:
- Name cannot be empty
- Email must be valid format if provided

### ExternalLink  
Represents additional resources linked to papers.

**Fields**:
- `type`: LinkType enum (github, dataset, website, pdf)
- `url`: string (required, valid URL)
- `description`: string (optional, max 200 chars)

**Validation Rules**:
- URL must be accessible (HTTP 200 status)
- GitHub links must point to valid repositories

### UserFavorites (Future Enhancement)
User's saved paper collection.

**Fields**:
- `user_id`: string (required when auth implemented)
- `paper_ids`: string[] (required)
- `created_at`: string (ISO timestamp)
- `notes`: Record<paper_id, string> (optional user notes per paper)

**Validation Rules**:
- All paper_ids must reference existing papers
- Maximum 1000 favorites per user

## Relationships

### Paper ↔ Author
- **Type**: Many-to-Many
- **Join**: Authors can appear on multiple papers, papers have multiple authors
- **Implementation**: Array of Author objects within Paper entity

### Paper ↔ Subject Area
- **Type**: Many-to-Many  
- **Join**: Papers can belong to multiple subject areas, areas contain multiple papers
- **Implementation**: Array of strings within Paper entity

### Paper ↔ Paper (Similarity)
- **Type**: Many-to-Many (computed)
- **Join**: Papers are related by semantic similarity of abstracts
- **Implementation**: Cosine similarity scores computed from embedding vectors

## Data Flow

### Ingestion Pipeline
```
MICCAI Website → Raw Paper Data → Embedding Generation → Search Index → Client JSON
```

### Search Pipeline  
```
User Query → Text Embedding → Vector Similarity → Filtered Results → Graph Visualization
```

### Favorites Pipeline (Future)
```
User Selection → Local Storage → Backend Sync → Database Persistence → Cross-Device Sync
```

## Storage Schema

### Static JSON Format (Phase 1)
```json
{
  "papers": [
    {
      "id": "paper-001",
      "title": "Deep Learning for Medical Image Segmentation",
      "abstract": "This paper presents...",
      "authors": [
        {
          "name": "Dr. Jane Smith",
          "affiliation": "University of Example",
          "email": "jane@example.edu"
        }
      ],
      "subject_areas": ["Computer Vision", "Medical Imaging"],
      "external_links": [
        {
          "type": "github",
          "url": "https://github.com/author/project",
          "description": "Source code and models"
        }
      ],
      "publication_date": "2025-01-15",
      "embedding_vector": [0.1, -0.3, 0.7, ...]
    }
  ],
  "metadata": {
    "total_papers": 856,
    "subject_areas": ["Computer Vision", "Machine Learning", "Medical Imaging"],
    "last_updated": "2025-09-14T10:00:00Z"
  }
}
```

### Database Schema (Future Enhancement)
```sql
CREATE TABLE papers (
    id VARCHAR(50) PRIMARY KEY,
    title VARCHAR(500) NOT NULL UNIQUE,
    abstract TEXT NOT NULL,
    subject_areas TEXT[], -- PostgreSQL array
    publication_date DATE NOT NULL,
    embedding_vector VECTOR(384), -- pgvector extension
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    affiliation VARCHAR(300),
    email VARCHAR(100)
);

CREATE TABLE paper_authors (
    paper_id VARCHAR(50) REFERENCES papers(id),
    author_id INT REFERENCES authors(id),
    author_order INT NOT NULL,
    PRIMARY KEY (paper_id, author_id)
);
```