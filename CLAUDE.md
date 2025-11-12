# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MilesHope.com is a static blog website built with Zola (a fast static site generator written in Rust). The site explores the intersection of spirituality, technology, and personal growth, with a focus on bazi, tarot, and mindful living.

**Tech Stack:**
- **Zola**: Static site generator for fast, efficient blog generation
- **Rust**: Custom tooling for Notion integration and automation (planned for Week 2)
- **Deployment**: Cloudflare Pages (planned for Week 5)
- **Content Source**: Notion database (integration planned)

## Essential Commands

### Development
```bash
# Start development server (auto-reload on changes)
zola serve

# Build site for production
zola build

# Check site for errors (validates links, frontmatter, etc.)
zola check
```

### Git
```bash
# Check status
git status

# Stage and commit changes
git add .
git commit -m "Your commit message"

# Push to remote (when configured)
git push origin main
```

## Project Structure

```
mileshope.com/
├── config.toml           # Main Zola configuration
├── content/              # All markdown content
│   ├── blog/            # Blog posts (synced from Notion)
│   │   └── _index.md    # Blog section configuration
│   ├── about.md         # About page
│   └── services.md      # Services page
├── templates/           # Tera templates (HTML)
│   ├── base.html        # Base layout (all pages extend this)
│   ├── index.html       # Homepage template
│   ├── section.html     # Blog listing template
│   ├── page.html        # Individual blog post template
│   ├── 404.html         # Error page
│   ├── tags/            # Tag taxonomy templates
│   └── categories/      # Category taxonomy templates
├── static/              # Static assets (copied as-is)
│   └── css/
│       └── style.css    # Main stylesheet
├── sass/                # Sass files (compiled by Zola)
└── public/              # Generated site (gitignored)
```

## Architecture Overview

### Template Hierarchy
- **base.html**: Root template with header, footer, and navigation. All other templates extend this.
- **index.html**: Homepage with hero section and recent posts grid
- **section.html**: Used for blog listing pages with pagination
- **page.html**: Individual blog post/page template with metadata, table of contents, and social sharing
- **Taxonomy templates**: Handle tags and categories with list and single views

### Content Flow
1. Markdown files in `content/` define pages and posts
2. Frontmatter (TOML between `+++`) sets metadata (title, date, tags, categories)
3. Zola processes templates with Tera templating engine
4. Static site generated in `public/` directory

### Key Zola Features Used
- **Taxonomies**: Automatic tag and category pages with RSS feeds
- **Search**: Built-in elasticlunr.js search index
- **RSS Feeds**: Auto-generated for main blog and taxonomies
- **Syntax Highlighting**: Code blocks with `base16-ocean-dark` theme
- **Sass Compilation**: CSS preprocessing support
- **Image Processing**: Planned for future optimization

## Content Management

### Blog Posts
Blog posts are stored in `content/blog/` with this frontmatter structure:

```toml
+++
title = "Post Title"
date = 2025-01-01
description = "Brief description for SEO"

[taxonomies]
categories = ["Category Name"]
tags = ["tag1", "tag2"]
+++
```

### Static Pages
Pages like About and Services don't require a `date` field in frontmatter. The template handles optional dates gracefully.

## Notion Integration (Week 2 - Implemented)

The `notion-sync/` directory contains a Rust CLI tool that syncs blog posts from Notion to Zola.

### Tool Structure
```
notion-sync/
├── src/
│   ├── main.rs          # CLI entry point and sync orchestration
│   ├── notion.rs        # Notion API client
│   ├── markdown.rs      # Markdown converter
│   └── frontmatter.rs   # Frontmatter generator
├── Cargo.toml           # Dependencies
├── .env.example         # Environment template
└── README.md            # Full documentation
```

### Usage

1. **Setup** (first time only):
   ```bash
   cd notion-sync
   cp .env.example .env
   # Edit .env with your NOTION_API_KEY and NOTION_DATABASE_ID
   ```

2. **Run sync**:
   ```bash
   cd notion-sync
   cargo run
   ```

3. **Build site** after sync:
   ```bash
   cd ..
   zola build
   ```

### Notion Database Requirements

Your Notion database must have these properties:
- **Name** or **Title** (Title type) - Post title
- **Status** (Status type) - Must include "Published" option
- **Published** or **Date** (Date type) - Publication date
- **Tags** (Multi-select, optional) - Post tags
- **Categories** (Multi-select, optional) - Post categories
- **Description** (Text, optional) - SEO description

### Supported Content

The tool converts these Notion blocks to Markdown:
- Paragraphs with formatting (bold, italic, code, strikethrough)
- Headings (H1, H2, H3)
- Bulleted and numbered lists
- Code blocks with syntax highlighting
- Quotes and callouts
- Links
- Dividers

See `notion-sync/README.md` for detailed setup instructions.

## Development Workflow

1. **Adding a blog post manually**: Create a new `.md` file in `content/blog/` with proper frontmatter
2. **Testing changes**: Run `zola serve` and visit `http://127.0.0.1:1111`
3. **Building for production**: Run `zola build` to generate static files in `public/`
4. **Template changes**: Modify files in `templates/` - changes auto-reload with `zola serve`
5. **Styling**: Edit `static/css/style.css` for immediate CSS changes

## Important Notes

- The `public/` directory is gitignored - it's regenerated on each build
- Zola uses Tera templating syntax (similar to Jinja2)
- Blog posts must have a `date` field for proper sorting
- Taxonomies (tags/categories) are automatically generated from frontmatter
- RSS feeds are auto-generated for blog and each taxonomy
- Site rebuilds are extremely fast (typically <100ms for small sites)

## Sprint Progress

**Week 1 Status**: ✅ Complete
- Zola installed and project structured
- Basic templates created (base, index, section, page, taxonomies, 404)
- Configuration set up with RSS, search, and taxonomies
- Sample blog post created
- Git repository initialized
- Site builds and serves successfully

**Week 2 Status**: ✅ Complete
- Rust CLI tool for Notion integration created
- Notion API client implemented with authentication
- Notion-to-Markdown converter built (supports headings, paragraphs, lists, code, quotes)
- Frontmatter generator implemented (TOML format for Zola)
- Sync script writes markdown files to `content/blog/` directory
- Comprehensive documentation added (notion-sync/README.md)
- Tool builds successfully and ready for testing

**Next Steps (Week 2 Testing)**: Test with live Notion database, then proceed to Week 3 (Design & Styling)
