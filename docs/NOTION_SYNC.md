# Notion Sync Guide

Guide for syncing blog posts from Notion to Zola using the custom Rust CLI tool.

## Overview

The `notion-sync` tool automatically converts Notion database pages into Zola-compatible Markdown files with proper frontmatter.

**Features:**
- Syncs only "Published" posts
- Converts Notion blocks to Markdown
- Generates proper Zola frontmatter
- Preserves formatting (bold, italic, code, etc.)
- Supports: headings, paragraphs, lists, code blocks, quotes

## Prerequisites

- Rust & Cargo installed
- Notion account with API access
- Notion integration created

## Initial Setup

### 1. Create Notion Integration

1. Go to https://www.notion.so/my-integrations
2. Click "+ New integration"
3. Name it "MilesHope Blog Sync"
4. Select your workspace
5. Click "Submit"
6. Copy the "Internal Integration Token"

### 2. Share Database with Integration

1. Open your Notion database for blog posts
2. Click "..." menu in top right
3. Select "Add connections"
4. Find and select your integration ("MilesHope Blog Sync")
5. Click "Confirm"

### 3. Get Database ID

From your Notion database URL:
```
https://www.notion.so/workspace/DATABASE_ID?v=VIEW_ID
                                 ^^^^^^^^^^^
```

Copy the `DATABASE_ID` part (32 characters, no dashes).

### 4. Configure Environment

```bash
cd notion-sync
cp .env.example .env
```

Edit `.env`:
```bash
NOTION_API_KEY=secret_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
NOTION_DATABASE_ID=your32characterdatabaseid
```

**Important:** Never commit `.env` to git (it's in `.gitignore`)

## Notion Database Setup

### Required Properties

Your Notion database **must** have these properties:

| Property | Type | Required | Purpose |
|----------|------|----------|---------|
| Name or Title | Title | ✅ Yes | Post title |
| Status | Status | ✅ Yes | Must include "Published" option |
| Published or Date | Date | ✅ Yes | Publication date |
| Tags | Multi-select | ❌ No | Post tags |
| Categories | Multi-select | ❌ No | Post categories |
| Description | Text | ❌ No | SEO description |

### Example Database Schema

```
┌─────────────┬──────────┬────────────┬──────┬────────────┬─────────────┐
│ Name (Title)│  Status  │ Published  │ Tags │ Categories │ Description │
├─────────────┼──────────┼────────────┼──────┼────────────┼─────────────┤
│ My First    │ Published│ 2025-01-13 │ rust │ Tech       │ A post      │
│ Post        │          │            │ blog │            │ about...    │
├─────────────┼──────────┼────────────┼──────┼────────────┼─────────────┤
│ Draft Post  │ Draft    │ 2025-01-14 │ ...  │ ...        │ ...         │
│ (Not synced)│          │            │      │            │             │
└─────────────┴──────────┴────────────┴──────┴────────────┴─────────────┘
```

**Notes:**
- Only pages with Status = "Published" are synced
- Date can be named "Published", "Date", or "Publish Date"
- Tags and Categories are optional
- Description is used for SEO meta tags

## Usage

### Syncing Posts

```bash
cd notion-sync
cargo run
```

**What happens:**
1. Connects to Notion API
2. Queries database for "Published" pages
3. Converts each page to Markdown
4. Generates Zola frontmatter
5. Writes files to `../content/blog/`
6. Shows summary of synced posts

**Example output:**
```
✅ Synced 5 posts from Notion to Zola
   - welcome-to-mileshope.md
   - getting-started-with-bazi.md
   - understanding-tarot.md
   - spiritual-practices.md
   - mindfulness-guide.md
```

### Viewing Generated Files

```bash
ls -l content/blog/
cat content/blog/welcome-to-mileshope.md
```

### Building Site After Sync

```bash
cd ..
zola build
# Or start dev server
zola serve
```

## Supported Notion Blocks

### ✅ Fully Supported

| Block Type | Markdown Output | Notes |
|------------|----------------|-------|
| Paragraph | Plain text | With formatting |
| Heading 1 | `# Heading` | - |
| Heading 2 | `## Heading` | - |
| Heading 3 | `### Heading` | - |
| Bulleted List | `- Item` | Nested lists supported |
| Numbered List | `1. Item` | Nested lists supported |
| Code Block | ` ```lang\ncode\n``` ` | Language preserved |
| Quote | `> Quote text` | - |
| Callout | `> 💡 Callout text` | Icon + text |
| Divider | `---` | - |
| Link | `[text](url)` | - |

### ✅ Text Formatting

| Format | Markdown | Example |
|--------|----------|---------|
| Bold | `**text**` | **bold** |
| Italic | `*text*` | *italic* |
| Code | `` `code` `` | `code` |
| Strikethrough | `~~text~~` | ~~strikethrough~~ |
| Link | `[text](url)` | [link](url) |

### ⚠️ Partially Supported

| Block Type | Status | Workaround |
|------------|--------|------------|
| Images | Not synced | Upload separately to `static/` |
| Files | Not synced | Upload separately |
| Embeds | Not synced | Use HTML in Markdown |
| Databases | Not synced | Export as tables |
| Toggle | Converted to heading | Loses toggle functionality |

### ❌ Not Supported

- Synced blocks
- Equations (LaTeX)
- Bookmarks
- Video embeds
- PDF embeds
- Advanced table formatting

## Frontmatter Generation

### Example Frontmatter

```toml
+++
title = "Welcome to MilesHope"
date = 2025-01-13
description = "Exploring spirituality, technology, and personal growth"

[taxonomies]
categories = ["Spirituality", "Tech"]
tags = ["welcome", "intro", "bazi"]
+++
```

### Frontmatter Fields

| Field | Source | Required |
|-------|--------|----------|
| `title` | Notion "Name" property | ✅ |
| `date` | Notion "Published" property | ✅ |
| `description` | Notion "Description" property | ❌ |
| `categories` | Notion "Categories" property | ❌ |
| `tags` | Notion "Tags" property | ❌ |

## Workflow Recommendations

### 1. Write in Notion First

**Pros:**
- Familiar interface
- Real-time collaboration
- Mobile app available
- Version history

### 2. Sync When Ready

```bash
cd notion-sync
cargo run
cd ..
```

### 3. Review Generated Markdown

```bash
# Check generated files
cat content/blog/latest-post.md

# Preview in dev server
zola serve
# Visit http://127.0.0.1:1111/blog/latest-post/
```

### 4. Make Adjustments

If needed, edit Markdown directly:
```bash
# Edit the generated file
vim content/blog/latest-post.md
```

**Note:** Manual edits will be overwritten on next sync!

### 5. Commit and Deploy

```bash
git add content/blog/
git commit -m "Add new blog post: Latest Post"
git push origin main
```

## Troubleshooting

### API Key Not Working

**Error:** `Unauthorized (401)`

**Solutions:**
1. Verify API key in `.env` is correct
2. Check integration hasn't been deleted
3. Ensure database is shared with integration

### Database Not Found

**Error:** `Database not found`

**Solutions:**
1. Verify DATABASE_ID in `.env` is correct (32 chars, no dashes)
2. Check database is shared with integration
3. Try re-sharing the database

### No Posts Synced

**Error:** `✅ Synced 0 posts`

**Solutions:**
1. Check posts have Status = "Published"
2. Verify Status property exists
3. Check "Published" option spelling (case-sensitive)

### Compilation Errors

**Error:** `cargo run` fails

**Solutions:**
```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build
```

### Missing Properties

**Error:** `Property 'X' not found`

**Solutions:**
1. Check property name spelling
2. Verify property type is correct
3. Add missing property to database

## Advanced Usage

### Custom Content Path

Edit `notion-sync/src/main.rs`:
```rust
const OUTPUT_DIR: &str = "../content/custom-path/";
```

### Filtering Posts

Edit query in `notion-sync/src/main.rs`:
```rust
// Only sync specific category
.filter(
    Property::new("Categories")
        .select()
        .equals("Tech")
)
```

### Custom Frontmatter

Edit `notion-sync/src/frontmatter.rs` to add custom fields.

## Best Practices

### ✅ Do

- Keep Notion database organized
- Use consistent naming
- Set Status to "Published" when ready
- Add descriptions for SEO
- Use tags and categories consistently
- Sync regularly

### ❌ Don't

- Commit `.env` file
- Edit synced Markdown directly (changes will be lost)
- Use unsupported block types extensively
- Forget to rebuild Zola after sync
- Change database schema without updating code

## API Rate Limits

**Notion API Limits:**
- 3 requests per second
- The tool includes automatic rate limiting

**If you hit limits:**
- Wait a few seconds
- Run sync again
- Notion will return after cooldown

## Debugging

### Enable Verbose Logging

```bash
RUST_LOG=debug cargo run
```

### Check API Response

Add to `notion-sync/src/main.rs`:
```rust
println!("API Response: {:?}", response);
```

### Test API Connection

```bash
curl -H "Authorization: Bearer $NOTION_API_KEY" \
     -H "Notion-Version: 2022-06-28" \
     "https://api.notion.com/v1/databases/$NOTION_DATABASE_ID"
```

## Resources

- [Notion API Docs](https://developers.notion.com/)
- [Notion Rust SDK](https://github.com/jakeswenson/notion)
- [Tool Source Code](../notion-sync/)

---

**Last Updated:** January 2025
