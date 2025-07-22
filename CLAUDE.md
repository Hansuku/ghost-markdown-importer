# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ghost Markdown Importer is a Rust CLI tool that converts Markdown files with YAML frontmatter into Ghost CMS import format. It supports both JSON export and ZIP package creation with images.

## Architecture

### Core Components

- **CLI**: `src/main.rs` - Command-line interface using clap for argument parsing
- **Library**: `src/lib.rs` - Public API exports for reusable components
- **Models**: 
  - `src/models/ghost.rs` - Ghost CMS JSON schema structures
  - `src/models/markdown.rs` - Markdown processing models and frontmatter parsing
- **Processors**:
  - `src/processors/markdown.rs` - Markdown file collection and processing
  - `src/processors/ghost_export.rs` - Ghost format generation and export
- **Utils**: `src/utils/file_ops.rs` - File system operations and utilities

### Data Flow

1. **Input Collection**: Scan directories for `.md` files (recursive optional)
2. **Markdown Processing**: Parse YAML frontmatter + markdown content → HTML
3. **Ghost Conversion**: Transform processed markdown into Ghost JSON schema
4. **Export**: Output as JSON file or ZIP package (with optional images)

## Development Commands

### Build & Run
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with sample data
cargo run -- ./examples/sample-content --verbose

# Install locally
cargo install --path .
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Watch mode for development
cargo install cargo-watch
cargo watch -x test
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Check for security issues
cargo audit
```

### Usage Examples
```bash
# Basic JSON export
gmi ./content --output ghost-import.json

# ZIP with images
gmi ./content --format zip --output blog.zip --include-images

# Advanced processing
gmi ./content --recursive --author "John Doe" --default-tags "tech,blog"
```

## Key Patterns

- **Error Handling**: Uses `anyhow::Result` for CLI and `thiserror` for library
- **Serialization**: Serde for JSON/YAML processing
- **File Operations**: Walkdir for recursive directory traversal
- **UUID Generation**: uuid crate for unique identifiers
- **Date Handling**: chrono for RFC3339 datetime formatting

## Input/Output Formats

### Input Markdown
```markdown
---
title: "Post Title"
date: 2024-01-15
author: "Author Name"
tags: ["tag1", "tag2"]
---

Content here...
```

### Output JSON
Ghost CMS compatible import format with posts, tags, users, and relationships.