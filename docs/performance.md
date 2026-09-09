# Performance

## Benchmarks (Typical Hardware)

- **Indexing Speed**: ~1000 files/minute
  - Depends on file size and complexity
  - Includes file I/O, hashing, chunking, embedding generation

- **Search Latency**: 20-30ms per query
  - ~95% recall with HNSW index
  - Sub-50ms for most queries

- **Memory Usage**:
  - Base: ~100MB
  - Embedding model: ~50MB
  - Per 10k chunks: ~40MB (embeddings + metadata)

- **Storage**:
  - Embeddings: ~1.5KB per chunk (384 floats)
  - Typical project (1000 files): ~75MB in Qdrant

## Optimization Tips

1. **Adjust chunk size**: Smaller chunks = more precise but slower indexing
2. **Use filters**: Pre-filter by language/extension for faster searches
3. **Batch processing**: Default 32 chunks per batch is optimal for most systems
4. **Incremental updates**: Use after initial index to save time
