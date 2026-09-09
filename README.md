# Project RAG - MCP Server for Code Understanding

[![Tests](https://img.shields.io/badge/tests-413%20passing-brightgreen)](https://github.com/Brainwires/project-rag)
[![Coverage](https://img.shields.io/badge/coverage-94%25-brightgreen)](https://github.com/Brainwires/project-rag)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/project-rag)](https://crates.io/crates/project-rag)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust-based Model Context Protocol (MCP) server that provides AI assistants with powerful RAG (Retrieval-Augmented Generation) capabilities for understanding massive codebases.

## Overview

This MCP server enables AI assistants to efficiently search and understand large projects by:
- Creating semantic embeddings of code files
- Storing them in a local vector database
- Providing fast semantic search capabilities
- Supporting incremental updates for efficiency

## Features

- **Local-First**: All processing happens locally using fastembed-rs (no API keys required)
- **Hybrid Search**: Combines vector similarity with BM25 keyword matching using Reciprocal Rank Fusion (RRF) for optimal results
- **AST-Based Chunking**: Uses Tree-sitter to extract semantic units (functions, classes, methods) for 12 languages
- **Comprehensive File Support**: Indexes 40+ file types including code, documentation (with PDF→Markdown conversion), and configuration files
- **Git History Search**: Search commit history with smart on-demand indexing (default: 10 commits, only indexes deeper as needed)
- **Multi-Project Support**: Index and query multiple codebases simultaneously with project filtering
- **Smart Indexing**: Automatically performs full indexing for new codebases or incremental updates for previously indexed ones
- **Cross-Process Locking**: Filesystem-based locks prevent multiple processes (e.g., multiple Claude Code sessions) from indexing the same codebase simultaneously
- **Stable Embedded Database**: LanceDB vector database (default, no external dependencies) with optional Qdrant support
- **Code Navigation**: Find definitions, references, and call graphs (lightweight LSP-like features)
- **Adaptive Search Thresholds**: Automatically lowers similarity threshold when no results found (0.7 → 0.6 → 0.5 → 0.4 → 0.3)
- **Slash Commands**: 9 convenient slash commands via MCP Prompts

## Documentation

- [Slash commands](docs/slash-commands.md) - the 9 `/project:*` commands
- [MCP tools](docs/mcp-tools.md) - the 9 tools, callable directly
- [Supported file types](docs/supported-file-types.md) - 40+ languages, docs, and config formats
- [Usage](docs/usage.md) - configuring Claude Code, Claude Desktop, GitHub Copilot CLI, and example tool calls
- [Configuration](docs/configuration.md) - environment variables, logging, embedding models, offline setup
- [Architecture](docs/architecture.md) - source tree and module layout
- [Technical details](docs/technical-details.md) - embeddings, hybrid search internals, LSP-like features, chunking
- [Index locking](docs/index-locking.md) - the concurrent-indexing protection system
- [Performance](docs/performance.md) - benchmarks and optimization tips
- [Status, limitations & roadmap](docs/status.md)
- [Troubleshooting](docs/troubleshooting.md)
- [Deployment](docs/deployment.md)
- [Architecture decision records](docs/adr/)

## Prerequisites

- **Rust**: 1.88+ with Rust 2024 edition support
- **protobuf-compiler**: Required for building (install via `sudo apt-get install protobuf-compiler` on Ubuntu/Debian)

### Vector Database Options

**LanceDB (Default - Embedded, Stable)**

No additional setup needed! LanceDB is an embedded vector database that runs directly in the application. It stores data in `./.lancedb` directory by default.

**Qdrant (Optional - Server-Based)**

To use Qdrant instead of LanceDB, build with the `qdrant-backend` feature:

```bash
cargo build --release --no-default-features --features qdrant-backend
```

Then start a Qdrant instance:

```bash
docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_data:/qdrant/storage \
    qdrant/qdrant
```

Or download standalone: https://qdrant.tech/documentation/guides/installation/

## Installation

```bash
# Navigate to the project
cd project-rag

# Install protobuf compiler (Ubuntu/Debian)
sudo apt-get install protobuf-compiler

# Build the release binary (with default LanceDB backend - stable and embedded!)
cargo build --release

# Or build with Qdrant backend (requires external server)
cargo build --release --no-default-features --features qdrant-backend

# The binary will be at target/release/project-rag
```

### ONNX Runtime (required at runtime)

Embeddings use `fastembed`'s `ort-load-dynamic` feature, which loads ONNX Runtime at *runtime*
via `dlopen` instead of statically linking it at build time (a prebuilt static binary needs a
newer libstdc++ than many environments provide). This means the binary builds fine without it,
but fails as soon as it tries to generate an embedding unless `libonnxruntime.so` is installed
and discoverable:

```bash
# Download and install the shared library (same version the Dockerfile/CI use)
curl -fsSL -o /tmp/onnxruntime.tgz \
  https://github.com/microsoft/onnxruntime/releases/download/v1.28.0/onnxruntime-linux-x64-1.28.0.tgz
tar -xzf /tmp/onnxruntime.tgz -C /tmp
sudo cp -P /tmp/onnxruntime-linux-x64-1.28.0/lib/libonnxruntime.so* /usr/local/lib/
sudo ldconfig
rm -rf /tmp/onnxruntime.tgz /tmp/onnxruntime-linux-x64-1.28.0

# Point project-rag at it (skip if it resolves via the default linker search path already)
export ORT_DYLIB_PATH=/usr/local/lib/libonnxruntime.so
```

## Quickstart

Run the server over stdio:

```bash
./target/release/project-rag
```

Add it to Claude Code:

```bash
cd /path/to/project-rag
claude mcp add project --command "$(pwd)/target/release/project-rag"
```

Restart Claude Code, then use any slash command (`/project:index`, `/project:query`, ...) or ask
in natural language. See [docs/usage.md](docs/usage.md) for Claude Desktop, GitHub Copilot CLI, and
example tool-call payloads.

## License

MIT License - see LICENSE file for details

## Contributing

Contributions welcome! Please ensure:

1. **Code Quality**: source files stay under 600 lines (enforced), code is formatted with
   `cargo fmt`, and clippy lints pass (`cargo clippy`).
2. **Testing**: add tests for new functionality, keep existing tests passing (`cargo test`), and
   update documentation.
3. **Commits**: clear, descriptive messages; one logical change per commit; reference issues where
   applicable.

See [docs/](docs/) for build/test commands and architecture details.

## Support

- **Issues**: https://github.com/Brainwires/project-rag/issues
- **Documentation**: See [docs/](docs/) for deployment, troubleshooting, and slash commands
- **Architecture**: See [docs/adr/](docs/adr/) for architecture decision records

## Acknowledgments

- **rmcp**: Official Rust Model Context Protocol SDK
- **Qdrant**: High-performance vector database
- **FastEmbed**: Fast local embedding generation
- **Claude**: For MCP protocol and testing

---

Built with ❤️ using Rust 2024 Edition
