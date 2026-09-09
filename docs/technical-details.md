# Technical Details

## Embeddings

- **Model**: all-MiniLM-L6-v2 (Sentence Transformers)
- **Dimensions**: 384
- **Library**: fastembed-rs with ONNX runtime
- **Performance**: ~500 embeddings/second

## Vector Database

- **Default**: LanceDB, embedded (no external server required)
- **Optional**: Qdrant, external server (`--no-default-features --features qdrant-backend`)
- **Distance Metric**: Cosine similarity
- **Index**: HNSW for fast approximate nearest neighbor search
- **Payload**: Stores file path, project, line numbers, language, hash, timestamp, content

## Hybrid Search

- **Vector Similarity**: Semantic understanding via embeddings (LanceDB or Qdrant)
- **Keyword Matching**: Full-text BM25 search via Tantivy inverted index
- **Fusion Algorithm**: Reciprocal Rank Fusion (RRF) with k=60 constant
- **BM25 Parameters**: Uses Tantivy's optimized BM25 implementation
- **Ranking**: RRF combines both rankings using 1/(k+rank) formula
- **Performance**: Both indexes queried in parallel for fast results

## Adaptive Threshold Logic

Both `query_codebase` and `search_by_filters` tools implement intelligent adaptive threshold lowering:

**How it works:**
1. Initial search uses the requested `min_score` threshold (default: 0.7)
2. If no results found and threshold > 0.3, automatically retries with lower thresholds
3. Fallback thresholds tried in order: 0.6 → 0.5 → 0.4 → 0.3
4. Response includes `threshold_used` and `threshold_lowered` fields for transparency

**Benefits:**
- Prevents empty results when semantic similarity is lower than expected
- Maintains search quality by preferring higher thresholds when possible
- Transparent: you always know the actual threshold used

**Example Response:**
```json
{
  "results": [...],
  "duration_ms": 45,
  "threshold_used": 0.4,
  "threshold_lowered": true
}
```

## Lightweight LSP Features

Project RAG provides code navigation capabilities similar to a Language Server Protocol (LSP) implementation, but optimized for semantic search use cases:

**Find Definition** (`find_definition`):
- Locate where symbols (functions, classes, variables) are defined
- Uses hybrid approach: high-precision stack-graphs for Python, TypeScript, Java, Ruby
- Falls back to AST-based RepoMap analysis for all other languages
- Reports precision level (High, Medium, Low) in results

**Find References** (`find_references`):
- Find all locations where a symbol is used across the codebase
- Categorizes reference types: Call, Read, Write, Import, TypeReference, Inheritance, Instantiation
- Useful for understanding how code is connected
- Option to include/exclude the definition site

**Get Call Graph** (`get_call_graph`):
- Analyze function call relationships
- Shows both callers (what calls this function) and callees (what this function calls)
- Configurable traversal depth for multi-level analysis
- Great for impact analysis and understanding code flow

**Architecture:**
```
RelationsProvider (trait)
├── StackGraphsProvider (high precision: ~95%)
│   └── Supports: Python, TypeScript, Java, Ruby
└── RepoMapProvider (fallback: ~70% precision)
    └── Supports: All tree-sitter languages (12+)
```

**When to Use:**
- **Find Definition**: "Where is this function defined?"
- **Find References**: "Where is this function called from?"
- **Get Call Graph**: "What functions does this code depend on?"

## Concurrent Indexing Protection

Project RAG prevents multiple processes (e.g. multiple Claude Code sessions) from indexing the
same codebase simultaneously, and guards the BM25 index against concurrent writes. See
[index-locking.md](index-locking.md) for the full lock-and-broadcast design (cross-process
filesystem locks, in-memory broadcast channels, stale-lock detection and cleanup).

## Code Chunking

- **Default**: Hybrid AST-based chunking
- **AST Support**: Rust, Python, JavaScript, TypeScript, Go, Java, Swift, C, C++, C#, Ruby, PHP
- **Fallback**: 50 lines per chunk for unsupported languages
- **Metadata**: Tracks start/end lines, language, file hash, project

## File Processing

- **Binary Detection**: 30% non-printable byte threshold (PDFs handled specially)
- **Language Detection**: 40+ file types supported (code, docs, configs)
- **PDF Processing**: Automatic text extraction and Markdown conversion with table preservation
- **Hash Algorithm**: SHA256 for change detection (works for all file types including PDFs)
- **.gitignore Support**: Uses `ignore` crate
