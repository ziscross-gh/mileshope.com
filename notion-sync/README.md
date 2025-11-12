# Notion to Zola Sync Tool

A Rust CLI tool that syncs blog posts from a Notion database to Zola markdown files.

## Features

- 📥 Fetches published pages from Notion database
- 📝 Converts Notion blocks to Markdown format
- 🏷️ Generates Zola-compatible frontmatter (TOML)
- 🔄 Syncs to `content/blog/` directory
- ⚡ Fast and efficient with proper error handling

## Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **Notion Integration** - Create at [notion.so/my-integrations](https://www.notion.so/my-integrations)
3. **Notion Database** - Set up with required properties (see below)

## Setup

### 1. Create Notion Integration

1. Go to [https://www.notion.so/my-integrations](https://www.notion.so/my-integrations)
2. Click "New integration"
3. Give it a name (e.g., "MilesHope Blog Sync")
4. Select your workspace
5. Copy the "Internal Integration Token" (starts with `secret_`)

### 2. Set Up Notion Database

Create a database with these properties:

| Property Name | Type | Required | Description |
|--------------|------|----------|-------------|
| Name/Title | Title | ✅ | Post title |
| Status | Status | ✅ | Must have "Published" option |
| Published | Date | ✅ | Publication date |
| Tags | Multi-select | ❌ | Post tags |
| Categories | Multi-select | ❌ | Post categories |
| Description | Text | ❌ | SEO description |

### 3. Share Database with Integration

1. Open your Notion database
2. Click "..." (more menu) in top right
3. Select "Add connections"
4. Find and add your integration

### 4. Configure Environment

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your credentials:
   ```env
   NOTION_API_KEY=secret_your_integration_token_here
   NOTION_DATABASE_ID=your_32_char_database_id
   OUTPUT_DIR=../content/blog
   ```

To get your database ID:
- Open the database in Notion
- Copy the URL, it looks like: `https://notion.so/workspace/[DATABASE_ID]?v=...`
- The DATABASE_ID is the 32-character hex string

## Usage

### Run the sync:

```bash
# From the notion-sync directory
cargo run

# Or with logging enabled
RUST_LOG=info cargo run
```

### Build release version:

```bash
cargo build --release
./target/release/notion-sync
```

## How It Works

1. **Fetch**: Queries Notion database for pages with Status = "Published"
2. **Convert**: Transforms Notion blocks to Markdown:
   - Headings (h1, h2, h3)
   - Paragraphs with formatting (bold, italic, code)
   - Lists (bulleted and numbered)
   - Code blocks with syntax highlighting
   - Quotes and callouts
3. **Generate Frontmatter**: Extracts metadata and creates TOML frontmatter
4. **Write**: Saves markdown files to `content/blog/` with slugified filenames

## Supported Notion Blocks

- ✅ Paragraphs
- ✅ Headings (H1, H2, H3)
- ✅ Bulleted lists
- ✅ Numbered lists
- ✅ Code blocks
- ✅ Quotes
- ✅ Callouts
- ✅ Dividers
- ✅ Text formatting (bold, italic, code, strikethrough)
- ✅ Links

## Output Format

Each synced post becomes a markdown file like:

```markdown
+++
title = "Your Post Title"
date = 2025-01-15
description = "Post description"

[taxonomies]
categories = ["Category1", "Category2"]
tags = ["tag1", "tag2", "tag3"]
+++

Your post content in Markdown format...
```

## Troubleshooting

### "NOTION_API_KEY environment variable not set"
- Make sure you created a `.env` file in the `notion-sync` directory
- Check that the file contains your integration token

### "No published pages found"
- Ensure your database has pages with Status = "Published"
- Verify your integration has access to the database
- Check that the NOTION_DATABASE_ID is correct

### API rate limits
- Notion API has rate limits (3 requests per second)
- The tool respects these limits automatically
- For large syncs, it may take a few minutes

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Future Enhancements

- [ ] Incremental sync (only updated pages)
- [ ] Image download and optimization
- [ ] Table support
- [ ] Nested list support
- [ ] Child pages/databases
- [ ] GitHub Actions automation
- [ ] Dry-run mode
- [ ] Selective sync (specific pages only)

## License

Copyright © 2025 Miles Hope. All rights reserved.
