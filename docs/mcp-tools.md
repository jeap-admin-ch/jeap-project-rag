# MCP Tools

The server provides 9 tools that can be used directly.

1. **index_codebase** - Smartly index a codebase directory
   - Automatically performs full indexing for new codebases
   - Automatically performs incremental updates for previously indexed codebases
   - Respects .gitignore and exclude patterns
   - Returns mode information (full or incremental)

2. **query_codebase** - Hybrid semantic + keyword search across the indexed code
   - Combines vector similarity with BM25 keyword matching (enabled by default)
   - Returns relevant code chunks with both vector and keyword scores
   - Configurable result limit and score threshold
   - Optional project filtering for multi-project setups

3. **get_statistics** - Get statistics about the indexed codebase
   - File counts, chunk counts, embedding counts
   - Language breakdown

4. **clear_index** - Clear all indexed data
   - Deletes the entire vector database collection
   - Prepares for fresh indexing

5. **search_by_filters** - Advanced hybrid search with filters
   - Always uses hybrid search for best results
   - Filter by file extensions (e.g., ["rs", "toml"])
   - Filter by programming languages
   - Filter by path patterns
   - Optional project filtering

6. **search_git_history** - Search git commit history using semantic search
   - Automatically indexes commits on-demand (default: 10 commits, configurable)
   - Searches commit messages, diffs, author info, and changed files
   - Smart caching: only indexes new commits as needed
   - Regex filtering by author name/email and file paths
   - Date range filtering (ISO 8601 or Unix timestamp)
   - Branch selection support

7. **find_definition** - Find where a symbol is defined (LSP-like)
   - Specify file path, line number, and column
   - Returns definition location with symbol metadata
   - Uses hybrid approach: high-precision stack-graphs (Python, TypeScript, Java, Ruby) or AST-based RepoMap fallback
   - Reports precision level of results

8. **find_references** - Find all references to a symbol
   - Specify file path, line number, and column
   - Returns all locations where the symbol is used
   - Categorizes reference types: Call, Read, Write, Import, TypeReference, Inheritance, Instantiation
   - Optional: include definition site in results

9. **get_call_graph** - Get call graph for a function
   - Specify file path, line number, and column for a function
   - Returns callers (what calls this function) and callees (what this function calls)
   - Configurable traversal depth (default: 1 level)
   - Useful for understanding code flow and impact analysis

See [usage.md](usage.md) for example calls, and [technical-details.md](technical-details.md) for
how adaptive thresholds and the LSP-like features work under the hood.
