# Usage

## Running as MCP Server

The server communicates over stdio following the MCP protocol:

```bash
./target/release/project-rag
```

## Configuring in Claude Code

Add the MCP server to Claude Code using the CLI:

```bash
# Navigate to the project directory first
cd /path/to/project-rag

# Add the MCP server to Claude Code
claude mcp add project --command "$(pwd)/target/release/project-rag"

# Or with logging enabled
claude mcp add project --command "$(pwd)/target/release/project-rag" --env RUST_LOG=info
```

After adding, restart Claude Code to load the server. The slash commands (`/project:index`, `/project:query`, etc.) will be available immediately.

## Configuring in Claude Desktop

Add to your Claude Desktop config:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "project-rag": {
      "command": "/absolute/path/to/project-rag/target/release/project-rag",
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

**Note**: Claude Code and Claude Desktop are different products with different configuration methods.

## Configuring in GitHub Copilot CLI

Inside a Copilot CLI session, run the interactive wizard:

```
/mcp add
```

Fill in:

- **Name:** `project-rag`
- **Command:** `/path/to/project-rag/target/release/project-rag`
- **Args:** `--model-path /path/to/models/all-MiniLM-L6-v2` (omit if you want fastembed to download the model)
- **Env:** `RUST_LOG=info`

Or edit `~/.copilot/mcp-config.json` (global) or `.mcp.json` in the workspace root (project-local) directly:

```json
{
  "mcpServers": {
    "project-rag": {
      "command": "/path/to/project-rag/target/release/project-rag",
      "args": ["--model-path", "/path/to/models/all-MiniLM-L6-v2"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

Then in a Copilot CLI session:

```
/mcp reload
/mcp show project-rag
```

For a one-shot test without persisting:

```bash
copilot --additional-mcp-config @/path/to/mcp-config.json
```

**Limitation**: Copilot CLI currently only supports MCP **tools**, not prompts. The 9 tools (`index_codebase`, `query_codebase`, `find_definition`, etc.) work normally, but the slash commands (`/project:index`, `/project:query`, ...) that Claude Code exposes as prompts will not be available. You can still ask Copilot in natural language ("index this repo", "find references to FastEmbedManager") and it will pick the right tool.

## Example Tool Usage

See [mcp-tools.md](mcp-tools.md) for the full tool reference. A few representative calls:

**Index a codebase:**
```json
{
  "path": "/path/to/your/project",
  "include_patterns": ["**/*.rs", "**/*.toml"],
  "exclude_patterns": ["**/target/**", "**/node_modules/**"],
  "max_file_size": 1048576
}
```
*Note: This automatically performs a full index for new codebases or an incremental update for previously indexed ones.*

**Query the codebase:**
```json
{
  "query": "How does authentication work?",
  "limit": 10,
  "min_score": 0.7
}
```

**Advanced filtered search:**
```json
{
  "query": "database connection pool",
  "limit": 5,
  "min_score": 0.75,
  "file_extensions": ["rs"],
  "languages": ["Rust"],
  "path_patterns": ["src/db"]
}
```

**Find definition of a symbol:**
```json
{
  "file_path": "/path/to/your/project/src/main.rs",
  "line": 42,
  "column": 10
}
```

**Find all references to a symbol:**
```json
{
  "file_path": "/path/to/your/project/src/lib.rs",
  "line": 15,
  "column": 8,
  "include_definition": false
}
```

**Get call graph for a function:**
```json
{
  "file_path": "/path/to/your/project/src/api.rs",
  "line": 100,
  "column": 4,
  "depth": 2
}
```
