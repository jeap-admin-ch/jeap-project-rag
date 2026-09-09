# Architecture

```
project-rag/
├── src/
│   ├── bm25_search.rs      # Tantivy BM25 keyword search with RRF fusion
│   ├── client/             # High-level client API
│   │   ├── mod.rs          # RagClient - unified interface for all operations
│   │   └── indexing/       # Indexing pipeline with progress reporting
│   ├── embedding/          # FastEmbed integration for local embeddings
│   │   ├── mod.rs          # EmbeddingProvider trait
│   │   └── fastembed_manager.rs  # all-MiniLM-L6-v2 implementation
│   ├── vector_db/          # Vector database implementations
│   │   ├── mod.rs          # VectorDatabase trait
│   │   ├── lance_client.rs # LanceDB + Tantivy hybrid search (default)
│   │   └── qdrant_client.rs  # Qdrant implementation (optional)
│   ├── indexer/            # File walking and code chunking
│   │   ├── mod.rs          # Module exports
│   │   ├── file_walker.rs  # Directory traversal with .gitignore + 40+ file types
│   │   ├── chunker.rs      # Chunking strategies (AST-based, fixed-lines, sliding window)
│   │   ├── ast_parser.rs   # Tree-sitter AST parsing for 12 languages
│   │   └── pdf_extractor.rs # PDF to Markdown converter with table support
│   ├── relations/          # Code relationship analysis (LSP-like features)
│   │   ├── mod.rs          # RelationsProvider trait, HybridRelationsProvider
│   │   ├── types.rs        # SymbolId, Definition, Reference, CallEdge types
│   │   ├── repomap/        # AST-based symbol extraction (fallback provider)
│   │   │   ├── mod.rs      # RepoMapProvider
│   │   │   ├── symbol_extractor.rs  # Extract definitions from AST
│   │   │   └── reference_finder.rs  # Find references via identifier matching
│   │   ├── storage/        # Relations storage layer
│   │   │   ├── mod.rs      # RelationsStore trait
│   │   │   └── lance_store.rs  # LanceDB storage (placeholder)
│   │   └── stack_graphs/   # Optional: High-precision name resolution
│   │       └── mod.rs      # StackGraphsProvider (feature-gated)
│   ├── mcp_server.rs       # MCP server with 9 tools
│   ├── types/              # Request/Response types with JSON schema
│   │   └── mod.rs          # All MCP request/response types
│   ├── main.rs             # Binary entry point with stdio transport
│   └── lib.rs              # Library root
├── Cargo.toml              # Rust 2024 edition with dependencies
├── README.md               # Project overview and quickstart
├── CONTRIBUTING.md         # Contributor guidelines
├── TESTING.md              # Testing guide
└── CLAUDE.md               # AI assistant instructions
```

See [technical-details.md](technical-details.md) for how the embedding/search/locking pieces work
internally, and [mcp-tools.md](mcp-tools.md) for the tool surface these modules back.
