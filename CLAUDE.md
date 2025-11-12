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

## Notion Integration (Planned - Week 2)

A Rust CLI tool will be built to:
1. Fetch content from Notion API
2. Convert Notion blocks to Markdown
3. Generate proper frontmatter
4. Sync to `content/blog/` directory

Tool structure (planned):
```
notion-to-zola/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── notion.rs        # Notion API client
│   ├── markdown.rs      # Markdown converter
│   └── frontmatter.rs   # Metadata handler
└── Cargo.toml
```

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

**Next Steps (Week 2)**: Build Notion integration Rust tool for automated content syncing
