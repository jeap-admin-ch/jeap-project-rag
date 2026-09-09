# Status, Limitations, and Roadmap

## Current Status

- Core architecture with modular design
- All 9 MCP tools implemented and working
- All 9 MCP slash commands implemented
- Hybrid search - Vector similarity + full BM25 with IDF
- AST-based chunking - Semantic code extraction for 12 languages
- Code navigation - Find definitions, references, and call graphs (LSP-like)
- Multi-project support - Index and query multiple codebases
- Persistent hash cache - Fast incremental updates across restarts
- Concurrent access protection - Smart lock management prevents index corruption
- FastEmbed integration for local embeddings
- LanceDB (default, embedded) and Qdrant (optional, external) vector database integration
- File walking with .gitignore support
- Language detection (40+ file types: code, docs, configs)
- PDF to Markdown conversion with table preservation
- SHA256-based change detection
- 413 unit tests passing (including relations, PDF extraction, BM25/RRF, adaptive threshold, cross-process locking, and lock safety tests)
- Hybrid relations provider - Stack-graphs for Python/TS/Java/Ruby, RepoMap fallback for all languages

## Known Limitations

- **Qdrant Backend**: Requires an external Qdrant server when using the `qdrant-backend` feature.
  The default LanceDB backend is fully embedded with no external dependencies. The Qdrant client
  itself needs the newer builder patterns (`UpsertPointsBuilder`, `SearchPointsBuilder`, etc.), all
  implemented.
- **FastEmbed Mutability**: Uses an unsafe workaround for mutable model access. Works correctly but
  should be refactored to use `Arc<Mutex<>>`.
- **Async Trait Warnings**: A handful of harmless warnings about `async fn` in public traits.
  Cosmetic issue, does not affect functionality.
- **Model Download**: First run downloads a ~50MB model (see
  [configuration.md](configuration.md#offline--behind-a-corporate-proxy) for pre-downloading it
  offline).
- **Path Filtering**: Currently post-query filtering (not optimized). Future: add Qdrant payload
  indexing for path patterns.
- **No Configuration File Format Beyond TOML**: Future: broader config format support.

### Scale Limitations

- **Large Codebases**: Projects with 100k+ files may take significant time to index. Mitigation:
  use incremental updates.
- **Memory**: Very large indexes (1M+ chunks) may require significant RAM. A typical project (5k
  files) uses under 500MB total.

## Future Enhancements

### High Priority
- [ ] Add comprehensive integration tests
- [ ] Configuration file support (TOML)
- [ ] Cache IDF statistics to disk for faster startup

### Medium Priority
- [ ] Embedded vector DB option (no external dependencies)
- [ ] Support for more embedding models
- [ ] Performance benchmarks and profiling
- [ ] AST support for more languages (Kotlin, Perl, Scala, etc.)

### Low Priority
- [ ] Web UI for testing/debugging
- [ ] Metrics and monitoring endpoints
- [ ] Multi-language documentation
- [ ] Alternative transport mechanisms (HTTP, WebSocket)
