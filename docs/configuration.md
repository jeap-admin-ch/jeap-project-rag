# Configuration

## Environment Variables

- `RUST_LOG` - Set logging level (options: `error`, `warn`, `info`, `debug`, `trace`)
  - Example: `RUST_LOG=debug cargo run`
  - Module filters are also supported, e.g. `RUST_LOG=project_rag=debug,lance=warn,ort=warn`
  - Logs are written to **stderr** (stdout is reserved for MCP JSON-RPC traffic), so it's safe to enable when running as an MCP server.

## Enabling Logs When Running as an MCP Server

When registering `project-rag` as an MCP server in Claude Code, pass `RUST_LOG`
via the env so you get useful logs without corrupting the protocol stream:

```bash
claude mcp add project-rag \
  --env RUST_LOG=info \
  -- /path/to/project-rag/target/release/project-rag \
     --model-path ~/models/all-MiniLM-L6-v2
```

(Some Claude Code versions use `-e KEY=VALUE` instead of `--env KEY=VALUE` —
run `claude mcp add --help` to confirm.)

View the captured stderr with the `/mcp` command in Claude Code, or read the
log files under `~/.cache/claude-cli-nodejs/<project>/mcp-logs-project-rag/`.

## Qdrant Configuration

- Currently hardcoded to `http://localhost:6334`
- Future: Add configuration file support

## Embedding Model

- Default: `all-MiniLM-L6-v2` (384 dimensions)
- First run downloads model (~50MB) to cache

## Offline / Behind a Corporate Proxy

If `fastembed`'s built-in download fails (e.g. behind a TLS-intercepting
corporate proxy with a private CA), or if you want to bake the model into a
container image, you can pre-download the model files yourself and point
`project-rag` at the directory.

Download the 5 required files for the default model:

```bash
mkdir -p ~/models/all-MiniLM-L6-v2
cd ~/models/all-MiniLM-L6-v2
BASE=https://huggingface.co/Qdrant/all-MiniLM-L6-v2-onnx/resolve/main
for f in model.onnx tokenizer.json config.json special_tokens_map.json tokenizer_config.json; do
  curl -fL -o "$f" "$BASE/$f"
done
```

Then start the server with the path. Any of the three forms work (CLI flag wins
over env var, which wins over the TOML config file):

```bash
# CLI flag
project-rag --model-path ~/models/all-MiniLM-L6-v2 serve

# Environment variable
export PROJECT_RAG_MODEL_PATH=~/models/all-MiniLM-L6-v2
project-rag serve

# TOML config: set embedding.model_path = "/path/to/model"
```

Note: `model_name` (default `all-MiniLM-L6-v2`) is still required because it
determines the embedding dimension. If you load a different model, set
`PROJECT_RAG_MODEL` (or `embedding.model_name` in the config) accordingly.

HuggingFace repos for the other supported models:

| Model name (config)        | HuggingFace repo                     | Dim |
| --------------------------- | ------------------------------------ | --- |
| `all-MiniLM-L6-v2`         | `Qdrant/all-MiniLM-L6-v2-onnx`       | 384 |
| `all-MiniLM-L12-v2`        | `Qdrant/all-MiniLM-L12-v2-onnx`      | 384 |
| `BAAI/bge-base-en-v1.5`    | `Qdrant/bge-base-en-v1.5-onnx-Q`     | 768 |
| `BAAI/bge-small-en-v1.5`   | `Qdrant/bge-small-en-v1.5-onnx-Q`    | 384 |

The directory must contain `model.onnx` (or `model_quantized.onnx`),
`tokenizer.json`, `config.json`, `special_tokens_map.json`, and
`tokenizer_config.json`.

Dockerfile example that bakes the model into the image:

```dockerfile
FROM rust:1.88 AS build
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /src/target/release/project-rag /usr/local/bin/project-rag
COPY models/all-MiniLM-L6-v2 /opt/models/all-MiniLM-L6-v2
ENV PROJECT_RAG_MODEL_PATH=/opt/models/all-MiniLM-L6-v2
ENTRYPOINT ["project-rag", "serve"]
```

## Chunking Strategy

- **Default**: Hybrid AST-based with fallback to fixed-lines
- **AST Parsing**: Extracts semantic units (functions, classes, methods) for Rust, Python, JavaScript, TypeScript, Go, Java, Swift, C, C++, C#, Ruby, PHP
- **Fallback**: 50 lines per chunk for unsupported languages
- **Alternative**: Sliding window with configurable overlap
