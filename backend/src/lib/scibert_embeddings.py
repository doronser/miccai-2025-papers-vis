#!/usr/bin/env python3
"""
SciBERT Embeddings Generator

Generates semantic embeddings for MICCAI papers using SciBERT model.
SciBERT is specifically trained on scientific literature and works better for academic papers.
"""

import json
import logging
import time
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import hashlib

import click
import numpy as np
import torch
from transformers import AutoTokenizer, AutoModel
from sklearn.metrics.pairwise import cosine_similarity

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class SciBERTEmbeddingGenerator:
    """SciBERT-based embedding generator for scientific papers."""

    def __init__(self, model_name: str = "sentence-transformers/all-MiniLM-L6-v2",
                 max_length: int = 512, batch_size: int = 8):
        """
        Initialize SciBERT embedding generator.

        Args:
            model_name: HuggingFace model identifier for SciBERT
            max_length: Maximum sequence length for tokenization
            batch_size: Batch size for processing multiple papers
        """
        self.model_name = model_name
        self.max_length = max_length
        self.batch_size = batch_size

        logger.info(f"Loading SciBERT model: {model_name}")
        self.tokenizer = None
        self.model = None
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    def load_model(self):
        """Load the SciBERT model and tokenizer."""
        if self.model is None:
            logger.info("Loading SciBERT tokenizer and model...")
            self.tokenizer = AutoTokenizer.from_pretrained(self.model_name)
            self.model = AutoModel.from_pretrained(self.model_name)
            self.model.to(self.device)
            self.model.eval()

            logger.info(f"Model loaded on device: {self.device}")
            logger.info(f"Model parameters: {sum(p.numel() for p in self.model.parameters()):,}")

    def create_text_for_embedding(self, paper: Dict) -> str:
        """
        Create text representation for embedding generation.

        Combines title and abstract with appropriate weighting for scientific papers.
        """
        title = paper.get('title', '')
        abstract = paper.get('abstract', '')

        # For scientific papers, title + abstract works well
        # Give more weight to title by including it twice
        text_parts = [title, title, abstract] if abstract else [title, title]
        text = ' '.join(text_parts).strip()

        # Truncate if too long (SciBERT has 512 token limit)
        if len(text.split()) > self.max_length // 2:  # Rough word-to-token ratio
            words = text.split()
            text = ' '.join(words[:self.max_length // 2])

        return text

    def generate_embedding(self, text: str) -> np.ndarray:
        """Generate embedding for a single text."""
        self.load_model()

        # Tokenize
        inputs = self.tokenizer(
            text,
            return_tensors="pt",
            truncation=True,
            padding=True,
            max_length=self.max_length
        ).to(self.device)

        # Generate embeddings
        with torch.no_grad():
            outputs = self.model(**inputs)
            # Use [CLS] token embedding (first token)
            embedding = outputs.last_hidden_state[:, 0, :].cpu().numpy()

        return embedding.squeeze()

    def generate_embeddings_batch(self, texts: List[str]) -> List[np.ndarray]:
        """Generate embeddings for multiple texts in batches."""
        self.load_model()

        embeddings = []

        for i in range(0, len(texts), self.batch_size):
            batch_texts = texts[i:i + self.batch_size]

            logger.info(f"Processing batch {i//self.batch_size + 1}/{(len(texts)-1)//self.batch_size + 1}")

            # Tokenize batch
            inputs = self.tokenizer(
                batch_texts,
                return_tensors="pt",
                truncation=True,
                padding=True,
                max_length=self.max_length
            ).to(self.device)

            # Generate embeddings
            with torch.no_grad():
                outputs = self.model(**inputs)
                batch_embeddings = outputs.last_hidden_state[:, 0, :].cpu().numpy()

            embeddings.extend([emb for emb in batch_embeddings])

            # Small delay to avoid overwhelming GPU
            time.sleep(0.1)

        return embeddings

    def process_papers_directory(self, papers_dir: str, embeddings_dir: str) -> Dict:
        """
        Process all papers from a directory and generate embeddings.

        Args:
            papers_dir: Directory containing individual paper JSON files
            embeddings_dir: Directory to save embedding files

        Returns:
            Dictionary with processing statistics
        """
        papers_path = Path(papers_dir)
        embeddings_path = Path(embeddings_dir)
        embeddings_path.mkdir(parents=True, exist_ok=True)

        logger.info(f"Processing papers from: {papers_path}")
        logger.info(f"Saving embeddings to: {embeddings_path}")

        # Find all paper JSON files
        paper_files = list(papers_path.glob("*.json"))
        paper_files = [f for f in paper_files if f.name != "index.json"]  # Skip index

        if not paper_files:
            logger.error(f"No paper files found in {papers_path}")
            return {"total_papers": 0, "processed": 0, "errors": 0}

        logger.info(f"Found {len(paper_files)} paper files")

        stats = {"total_papers": len(paper_files), "processed": 0, "errors": 0, "skipped": 0}

        for i, paper_file in enumerate(paper_files, 1):
            try:
                # Check if embedding already exists
                embedding_file = embeddings_path / f"{paper_file.stem}_embedding.npz"
                if embedding_file.exists():
                    stats["skipped"] += 1
                    if i % 100 == 0:
                        logger.info(f"Progress: {i}/{len(paper_files)} (skipping existing)")
                    continue

                # Load paper
                with open(paper_file, 'r', encoding='utf-8') as f:
                    paper = json.load(f)

                # Generate text for embedding
                text = self.create_text_for_embedding(paper)

                if not text.strip():
                    logger.warning(f"Empty text for paper {paper_file.stem}")
                    stats["errors"] += 1
                    continue

                # Generate embedding
                embedding = self.generate_embedding(text)

                # Save embedding with metadata
                embedding_data = {
                    'paper_id': paper.get('id', paper_file.stem),
                    'embedding': embedding,
                    'text_used': text,
                    'model_name': self.model_name,
                    'generated_at': time.strftime("%Y-%m-%d %H:%M:%S"),
                    'text_hash': hashlib.md5(text.encode()).hexdigest()
                }

                np.savez_compressed(embedding_file, **embedding_data)

                stats["processed"] += 1

                # Progress reporting
                if i % 10 == 0:
                    logger.info(f"Progress: {i}/{len(paper_files)} papers processed")

                # Rate limiting for API/model calls
                time.sleep(0.05)  # Small delay between papers

            except Exception as e:
                logger.error(f"Error processing {paper_file}: {e}")
                stats["errors"] += 1
                continue

        # Save processing stats
        stats_file = embeddings_path / "generation_stats.json"
        with open(stats_file, 'w', encoding='utf-8') as f:
            json.dump(stats, f, indent=2)

        logger.info(f"✅ Processing complete:")
        logger.info(f"   Total papers: {stats['total_papers']}")
        logger.info(f"   Processed: {stats['processed']}")
        logger.info(f"   Skipped (existing): {stats['skipped']}")
        logger.info(f"   Errors: {stats['errors']}")

        return stats

    def load_embedding(self, embedding_file: str) -> Optional[Dict]:
        """Load a single embedding file."""
        try:
            data = np.load(embedding_file, allow_pickle=True)
            return {
                'paper_id': str(data['paper_id']),
                'embedding': data['embedding'],
                'text_used': str(data['text_used']),
                'model_name': str(data['model_name']),
                'generated_at': str(data['generated_at']),
                'text_hash': str(data['text_hash'])
            }
        except Exception as e:
            logger.error(f"Error loading {embedding_file}: {e}")
            return None

    def compute_similarity(self, embedding1: np.ndarray, embedding2: np.ndarray) -> float:
        """Compute cosine similarity between two embeddings."""
        # Reshape for sklearn
        emb1 = embedding1.reshape(1, -1)
        emb2 = embedding2.reshape(1, -1)

        similarity = cosine_similarity(emb1, emb2)[0, 0]
        return float(similarity)

    def find_similar_papers(self, target_paper_id: str, embeddings_dir: str,
                          top_k: int = 10, min_similarity: float = 0.3) -> List[Tuple[str, float]]:
        """
        Find papers similar to the target paper.

        Returns list of (paper_id, similarity_score) tuples.
        """
        embeddings_path = Path(embeddings_dir)

        # Load target embedding
        target_file = embeddings_path / f"{target_paper_id}_embedding.npz"
        if not target_file.exists():
            logger.error(f"Target embedding not found: {target_file}")
            return []

        target_data = self.load_embedding(str(target_file))
        if not target_data:
            return []

        target_embedding = target_data['embedding']

        # Load all other embeddings and compute similarities
        similarities = []

        for embedding_file in embeddings_path.glob("*_embedding.npz"):
            if embedding_file.stem.replace('_embedding', '') == target_paper_id:
                continue  # Skip self

            other_data = self.load_embedding(str(embedding_file))
            if not other_data:
                continue

            similarity = self.compute_similarity(target_embedding, other_data['embedding'])

            if similarity >= min_similarity:
                paper_id = embedding_file.stem.replace('_embedding', '')
                similarities.append((paper_id, similarity))

        # Sort by similarity and return top K
        similarities.sort(key=lambda x: x[1], reverse=True)
        return similarities[:top_k]


# CLI Interface
@click.command()
@click.option('--papers-dir', default='src/data/papers_by_id',
              help='Directory containing paper JSON files')
@click.option('--embeddings-dir', default='src/data/embeddings_by_id',
              help='Directory to save embedding files')
@click.option('--model-name', default='sentence-transformers/all-MiniLM-L6-v2',
              help='Embedding model name from HuggingFace')
@click.option('--batch-size', default=8, type=int,
              help='Batch size for processing')
@click.option('--max-length', default=512, type=int,
              help='Maximum sequence length')
@click.option('--test-single', is_flag=True, help='Test with single paper only')
@click.option('--verbose', '-v', is_flag=True, help='Enable verbose logging')
def main(papers_dir: str, embeddings_dir: str, model_name: str,
         batch_size: int, max_length: int, test_single: bool, verbose: bool):
    """
    SciBERT Embeddings Generator

    Generates semantic embeddings for MICCAI papers using SciBERT model.
    SciBERT is specifically trained on scientific literature.

    Examples:
        # Generate embeddings for all papers
        python -m src.lib.scibert_embeddings

        # Use custom directories and model
        python -m src.lib.scibert_embeddings --papers-dir data/papers --embeddings-dir data/embeddings

        # Smaller batch size for limited GPU memory
        python -m src.lib.scibert_embeddings --batch-size 4
    """
    if verbose:
        logging.getLogger().setLevel(logging.DEBUG)

    # Check if GPU is available
    if torch.cuda.is_available():
        logger.info(f"🚀 Using GPU: {torch.cuda.get_device_name()}")
    else:
        logger.info("💻 Using CPU (GPU not available)")

    generator = SciBERTEmbeddingGenerator(
        model_name=model_name,
        max_length=max_length,
        batch_size=batch_size
    )

    try:
        if test_single:
            # Test with just one paper
            papers_path = Path(papers_dir)
            paper_files = list(papers_path.glob("*.json"))
            paper_files = [f for f in paper_files if f.name != "index.json"]

            if paper_files:
                test_paper = paper_files[0]
                click.echo(f"🧪 Testing with single paper: {test_paper.name}")

                # Load and process single paper
                with open(test_paper, 'r', encoding='utf-8') as f:
                    paper = json.load(f)

                text = generator.create_text_for_embedding(paper)
                click.echo(f"📄 Paper title: {paper.get('title', 'Unknown')}")
                click.echo(f"📏 Text length: {len(text)} characters")

                # Generate embedding
                embedding = generator.generate_embedding(text)
                click.echo(f"🔢 Embedding shape: {embedding.shape}")
                click.echo(f"✅ Single paper test successful!")

                # Save test embedding
                embeddings_path = Path(embeddings_dir)
                embeddings_path.mkdir(parents=True, exist_ok=True)
                test_file = embeddings_path / f"{test_paper.stem}_test_embedding.npz"

                np.savez_compressed(test_file,
                    paper_id=paper.get('id', test_paper.stem),
                    embedding=embedding,
                    text_used=text,
                    model_name=model_name)

                click.echo(f"💾 Test embedding saved to: {test_file}")
                return
            else:
                click.echo("❌ No paper files found for testing")
                return

        stats = generator.process_papers_directory(papers_dir, embeddings_dir)

        if stats["processed"] > 0:
            click.echo(f"✅ Successfully generated embeddings for {stats['processed']} papers")
            click.echo(f"📁 Embeddings saved to: {embeddings_dir}")
            click.echo(f"📊 Stats: {stats['processed']} processed, {stats['skipped']} skipped, {stats['errors']} errors")
        else:
            click.echo("❌ No embeddings were generated")

    except KeyboardInterrupt:
        click.echo("\n⚠️  Generation interrupted by user")
    except Exception as e:
        click.echo(f"❌ Error: {e}")
        raise


if __name__ == "__main__":
    main()