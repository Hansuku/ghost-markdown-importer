# Ghost Markdown Importer

A powerful Rust CLI tool that converts Markdown files to Ghost CMS import format (JSON or ZIP package).

## Features

- ✅ **Ghost CMS Compatible**: Generates perfect Ghost import JSON format
- 📝 **Markdown Processing**: Full markdown to HTML conversion with frontmatter support
- 📦 **Flexible Export**: JSON or ZIP format with optional image inclusion
- 🔄 **Recursive Processing**: Process entire directory trees
- 🎯 **Frontmatter Support**: YAML frontmatter for post metadata
- 🏷️ **Tag Management**: Automatic tag creation and assignment
- 👤 **Author Support**: Multiple authors with default fallback
- 🔍 **File Filtering**: Exclude patterns and verbose logging

## Installation

### From Source
```bash
git clone https://github.com/Hansuku/ghost-markdown-importer.git
cd ghost-markdown-importer
cargo install --path .
```

### From Crates.io (when published)
```bash
cargo install ghost-markdown-importer
```

## Usage

### Basic Usage

```bash
# Convert a folder of markdown files to Ghost JSON
gmi ./my-blog-posts

# Output to specific file
gmi ./content --output ghost-import.json

# Create ZIP package with images
gmi ./content --output blog-export.zip --format zip --include-images

# Process recursively with custom settings
gmi ./content --recursive --author "hansuku" --default-tags "tech,blog,rust"
```

### Advanced Usage

```bash
# Verbose processing with exclusions
gmi ./content \
  --recursive \
  --verbose \
  --author "hansuku" \
  --default-tags "development,programming" \
  --exclude "drafts/*,old/*" \
  --output ghost-export.json

# Complete ZIP package
gmi ./content \
  --format zip \
  --output complete-blog.zip \
  --include-images \
  --recursive \
  --author "Blog Author" \
  --default-tags "featured"
```

## Input Format

### Markdown Files

Your markdown files should use YAML frontmatter for metadata:

```markdown
---
title: "My Awesome Post"
date: 2024-01-15
author: "hansuku"
tags: ["rust", "tutorial", "programming"]
slug: "my-awesome-post"
description: "This post is about something amazing"
featured: true
image: "/images/awesome.jpg"
status: "published"
---

# Hello World

This is the content of your post. All markdown features are supported:

- **Bold text**
- *Italic text*
- `Code blocks`
- [Links](https://example.com)
- Images: ![Alt text](/images/example.png)

## Code Blocks

```rust
fn main() {
    println!("Hello, Ghost!");
}
```

## Tables

| Feature | Supported |
|---------|-----------|
| Markdown | ✅ |
| Images | ✅ |
| Frontmatter | ✅ |
```

### Supported Frontmatter Fields

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Post title (required) |
| `date` | string/date | Publication date |
| `author` | string | Author name |
| `tags` | array | List of tags |
| `slug` | string | URL slug (auto-generated if missing) |
| `description` | string | Post excerpt |
| `featured` | boolean | Mark as featured post |
| `image` | string | Featured image URL |
| `status` | string | `published`, `draft`, or `scheduled` |
| `category` | string | Primary category |

## Output Formats

### JSON Export
Creates a single `.json` file ready for Ghost import:

```json
{
  "meta": {
    "exported_on": 1704312169707,
    "version": "5.75.1"
  },
  "data": {
    "posts": [
      {
        "id": 1,
        "title": "My Awesome Post",
        "slug": "my-awesome-post",
        "html": "<h1>Hello World</h1>...",
        "status": "published",
        "published_at": "2024-01-15T00:00:00.000Z",
        ...
      }
    ],
    "tags": [...],
    "users": [...]
  }
}
```

### ZIP Export
Creates a complete package with:
- `ghost-import.json` - JSON import file
- `content/images/` - All images organized by directory structure

## CLI Options

```
USAGE:
    gmi [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Input directory containing markdown files

OPTIONS:
    -o, --output <OUTPUT>          Output file path (JSON or ZIP)
    -f, --format <FORMAT>         Export format [default: json] [possible values: json, zip]
    -r, --recursive               Process directories recursively
    -a, --author <AUTHOR>         Default author name for posts without authors
        --default-tags <TAGS>...   Default tags to add to all posts
        --exclude <PATTERNS>...    Exclude files matching these patterns
        --include-images          Include images in ZIP export
    -v, --verbose                 Verbose output
    -h, --help                    Print help
```

## Examples

### Example Directory Structure
```
my-blog/
├── posts/
│   ├── 2024-01-15-hello-world.md
│   ├── 2024-01-20-rust-tutorial.md
│   └── drafts/
│       └── 2024-01-25-draft-post.md
├── images/
│   ├── hello-world.png
│   └── rust-logo.png
└── assets/
    └── css/
```

### Processing Example
```bash
# Process all posts, exclude drafts
gmi ./my-blog/posts \
  --recursive \
  --exclude "drafts/*" \
  --author "hansuku" \
  --default-tags "blog,tech" \
  --output ghost-blog.json \
  --verbose

# Create complete ZIP package with images
gmi ./my-blog \
  --recursive \
  --format zip \
  --output complete-blog.zip \
  --include-images \
  --author "hansuku" \
  --default-tags "featured"
```

## Importing to Ghost

### Method 1: Ghost Admin
1. Go to your Ghost admin panel
2. Navigate to **Settings** → **Labs**
3. Scroll to **Import content**
4. Upload the generated `ghost-import.json` or `.zip` file
5. Click **Import**

### Method 2: Ghost CLI (if available)
```bash
ghost import ghost-import.json
```

## Development

### Prerequisites
- Rust 1.70+
- Cargo

### Building
```bash
# Clone the repository
git clone https://github.com/Hansuku/ghost-markdown-importer.git
cd ghost-markdown-importer

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with sample data
cargo run -- ./examples/sample-content --verbose
```

### Download Pre-built Binary
Download the latest release binary from the [releases page](https://github.com/Hansuku/ghost-markdown-importer/releases):

- **Linux x86_64**: `gmi-linux-x86_64.tar.gz`
- **macOS x86_64**: `gmi-macos-x86_64.tar.gz`
- **macOS ARM64**: `gmi-macos-arm64.tar.gz`
- **Windows x86_64**: `gmi-windows-x86_64.zip`

### Installation from Binary
```bash
# Linux/macOS
tar -xzf gmi-linux-x86_64.tar.gz
sudo mv gmi /usr/local/bin/

# Windows
# Extract the zip file and move gmi.exe to a directory in your PATH
```

## Usage

### Basic Usage

```bash
# Convert a folder of markdown files to Ghost JSON
gmi ./my-blog-posts

# Output to specific file
gmi ./content --output ghost-import.json

# Create ZIP package with images
gmi ./content --output blog-export.zip --format zip --include-images

# Process recursively with custom settings
gmi ./content --recursive --author "hansuku" --default-tags "tech,blog,rust"

### Project Structure
```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── models/
│   ├── ghost.rs         # Ghost JSON structures
│   └── markdown.rs      # Markdown processing models
├── processors/
│   ├── markdown.rs      # Markdown file processing
│   └── ghost_export.rs  # Ghost format generation
└── utils/
    └── file_ops.rs      # File utilities
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

### Development Setup
```bash
# Install development dependencies
cargo install cargo-watch

# Run tests in watch mode
cargo watch -x test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Troubleshooting

### Common Issues

**"No markdown files found"**
- Ensure your files have `.md` extension
- Check directory path is correct
- Use `--recursive` flag for subdirectories

**"Failed to parse YAML frontmatter"**
- Check YAML syntax in frontmatter
- Ensure proper YAML indentation
- Validate special characters in strings

**"Images not included in ZIP"**
- Use `--include-images` flag
- Ensure images are in supported formats (jpg, png, gif, webp, svg)
- Check image paths are accessible

### Debug Mode
Use `--verbose` flag for detailed processing information:
```bash
gmi ./content --verbose
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) for markdown parsing
- [serde](https://serde.rs/) for JSON serialization
- [clap](https://docs.rs/clap/) for CLI argument parsing
- Ghost CMS team for the excellent import format documentation

## Changelog

### v0.1.0
- Initial release
- Markdown to Ghost JSON conversion
- ZIP export with images
- CLI interface with comprehensive options
- YAML frontmatter support
- Tag and author management