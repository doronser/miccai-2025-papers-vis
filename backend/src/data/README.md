# Test Data Directory

This directory contains the test data files used by the DataLoader tests.

## Structure

- `papers_by_id/` - Directory containing individual paper JSON files
  - `index.json` - Index file listing all papers (not tracked in git due to size)
  - `miccai-*.json` - Individual paper metadata files (not tracked in git due to size)
- `embeddings_by_id/` - Directory containing paper embedding NPZ files
  - `miccai-*_embedding.npz` - Embedding files for papers (not tracked in git due to size)

## Getting the Data

The data files are not committed to this repository due to their size (2000+ files, ~100MB+).

To run the tests, you need to obtain the data files from one of these sources:

1. **From the original Python repository**: Copy the `backend/src/data/` directory from the source repository at `/l2l/src/miccai-2025-papers-vis/backend/src/data/`

2. **For CI/Testing**: Tests will be skipped if data files are not present, or data should be downloaded/mounted during CI runs.

## Data Files Excluded from Git

The following patterns are excluded via `.gitignore`:
- `backend/src/data/papers_by_id/*.json` - All paper JSON files
- `backend/src/data/embeddings_by_id/*.npz` - All embedding NPZ files

Only the directory structure (`.gitkeep` files) and this README are tracked.
