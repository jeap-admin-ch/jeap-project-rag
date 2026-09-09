# Supported File Types

Project RAG automatically indexes and searches **40+ file types** across three categories.

## Programming Languages (24 languages)

Supports AST-based semantic chunking for these languages:
- **Rust** (`.rs`)
- **Python** (`.py`)
- **JavaScript** (`.js`, `.mjs`, `.cjs`), **TypeScript** (`.ts`), **JSX** (`.jsx`), **TSX** (`.tsx`)
- **Go** (`.go`)
- **Java** (`.java`)
- **C** (`.c`), **C++** (`.cpp`, `.cc`, `.cxx`), **C/C++ Headers** (`.h`, `.hpp`)
- **C#** (`.cs`)
- **Swift** (`.swift`)
- **Kotlin** (`.kt`, `.kts`)
- **Scala** (`.scala`)
- **Ruby** (`.rb`)
- **PHP** (`.php`)
- **Shell** (`.sh`, `.bash`)
- **SQL** (`.sql`)
- **HTML** (`.html`, `.htm`)
- **CSS** (`.css`), **SCSS** (`.scss`, `.sass`)

## Documentation Formats (8 formats)

With special handling for rich content:
- **Markdown** (`.md`, `.markdown`)
- **PDF** (`.pdf`) - **Automatically converted to Markdown** with table preservation
- **reStructuredText** (`.rst`)
- **AsciiDoc** (`.adoc`, `.asciidoc`)
- **Org Mode** (`.org`)
- **Plain Text** (`.txt`)
- **Log Files** (`.log`)

**PDF Conversion Features:**
- Extracts text content using `pdf-extract` library
- Converts to Markdown format automatically
- Preserves **table structures** (detects tab/space-separated columns)
- Detects and formats **headings** (ALL CAPS lines and section markers)
- Handles multi-column layouts intelligently
- Chunks like any other text file (50 lines per chunk by default)

## Configuration Files (8 formats)

For complete project understanding:
- **JSON** (`.json`)
- **YAML** (`.yaml`, `.yml`)
- **TOML** (`.toml`)
- **XML** (`.xml`)
- **INI** (`.ini`)
- **Config files** (`.conf`, `.config`, `.cfg`)
- **Properties** (`.properties`)
- **Environment** (`.env`)

## Example Use Cases

```bash
# Index documentation PDFs in your project
query_codebase("API authentication flow")  # Finds content in .pdf, .md, .rst files

# Search configuration files
query_codebase("database connection string")  # Finds .yaml, .toml, .env, .conf files

# Find code implementations
search_by_filters(query="JWT validation", file_extensions=["rs", "go"])
```
