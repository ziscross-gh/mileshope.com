# Notion to Zola Sync Tool

A simple Python script that syncs published blog posts from your Notion database to Zola markdown files.

## Features

- Fetches only posts with Status = "Published"
- Converts Notion blocks to Zola-compatible markdown
- Generates proper TOML frontmatter
- Supports rich text formatting (bold, italic, code, links)
- Handles headings, lists, code blocks, quotes, callouts, images
- Automatic slug generation from post titles

## Setup

### 1. Create a Notion Integration

1. Go to https://www.notion.so/my-integrations
2. Click "New integration"
3. Name it (e.g., "Zola Blog Sync")
4. Copy the "Internal Integration Token"

### 2. Share Your Database

1. Open your Notion database
2. Click the "..." menu in the top right
3. Click "Add connections"
4. Select your integration

### 3. Get Your Database ID

Your database URL looks like:
```
https://www.notion.so/your-workspace/DATABASE_ID?v=VIEW_ID
```

Copy the `DATABASE_ID` part (32-character string).

### 4. Configure Environment Variables

Copy the example file:
```bash
cp .env.example .env
```

Edit `.env` and add your credentials:
```bash
NOTION_API_KEY=secret_your_api_key_here
NOTION_DATABASE_ID=your_database_id_here
```

### 5. Install Dependencies

```bash
pip3 install requests
```

Or with a virtual environment (recommended):
```bash
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install requests
```

## Notion Database Structure

Your Notion database should have these properties (matches MilesHope.com structure):

### Required Properties
- **Title** (Title type) - The blog post title
- **Status** (Text type) - Must be exactly "Published" to sync
- **Publish Date** (Date type) - Publication date
- **Content** (Text type) - The main blog post content

### Optional Properties
- **SEO Description** (Text type) - Meta description for SEO
- **Tags** (Text type) - Space or comma-separated tags (e.g., "tag1 tag2" or "tag1, tag2")
- **Category** (Text type) - Space or comma-separated categories
- **Slug** (Text type) - URL slug (auto-generated from title if empty)
- **Excerpt** (Text type) - Short excerpt
- **Featured Image**, **Notes**, etc. - Other fields for your reference

## Usage

### Run the sync:
```bash
# If using .env file
source .env  # On Windows: set commands or use dotenv
python3 sync.py

# Or set variables inline
NOTION_API_KEY=secret_xxx NOTION_DATABASE_ID=xxx python3 sync.py
```

### After syncing:
```bash
# Preview your site
zola serve

# Build for production
zola build
```

## Supported Content

The tool converts these Notion blocks to Markdown:

- **Paragraphs** with rich text (bold, italic, code, strikethrough, links)
- **Headings** (H1, H2, H3)
- **Lists** (bulleted and numbered)
- **Code blocks** with syntax highlighting
- **Quotes**
- **Callouts** (converted to blockquotes with emoji)
- **Dividers**
- **Images** (external and uploaded)

## Output

Posts are written to `content/blog/` with this structure:

```markdown
+++
title = "Your Post Title"
date = 2025-01-12
description = "Optional description"

[taxonomies]
categories = ["Category 1", "Category 2"]
tags = ["tag1", "tag2"]
+++

Your post content here...
```

Filenames are auto-generated from post titles (e.g., "My Post" → "my-post.md").

## Troubleshooting

### "Error communicating with Notion API"
- Check your `NOTION_API_KEY` is correct
- Verify your `NOTION_DATABASE_ID` is correct
- Ensure your integration has access to the database (Step 2)

### "No published posts found"
- Verify your database has a "Status" property
- Check at least one post has Status = "Published"
- Make sure the status option is exactly "Published" (case-sensitive)

### Posts not appearing on site
- Run `zola serve` to preview
- Check the generated files in `content/blog/`
- Ensure posts have valid frontmatter (especially `date` field)

## Automation

You can automate syncing with:

### Cron job (Unix/Mac)
```bash
# Run every hour
0 * * * * cd /path/to/mileshope.com && source .env && python3 sync.py
```

### GitHub Actions (for auto-deploy)
Create `.github/workflows/sync.yml`:
```yaml
name: Sync and Deploy
on:
  schedule:
    - cron: '0 * * * *'  # Every hour
  workflow_dispatch:  # Manual trigger

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Sync from Notion
        env:
          NOTION_API_KEY: ${{ secrets.NOTION_API_KEY }}
          NOTION_DATABASE_ID: ${{ secrets.NOTION_DATABASE_ID }}
        run: |
          pip install requests
          python sync.py
      - name: Build site
        run: |
          # Install Zola and build
          # Deploy to your hosting
```

## Development

The script is a single file (`sync.py`) with no external dependencies except `requests`. Feel free to customize:

- Modify `block_to_markdown()` to add more block type support
- Adjust `create_frontmatter()` for different property mappings
- Change `slugify()` for different URL slug generation
- Add error handling or logging as needed
