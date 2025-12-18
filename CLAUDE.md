# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MilesHope.com is a static blog website built with Zola (a fast static site generator written in Rust). The site explores the intersection of spirituality, technology, and personal growth, with a focus on bazi, tarot, and mindful living.

**Tech Stack:**
- **Zola**: Static site generator for fast, efficient blog generation
- **Tailwind CSS v4**: Utility-first CSS framework (standalone CLI)
- **Python**: Simple sync script for Notion integration
- **Cloudflare Pages**: Live deployment at https://www.mileshope.com
- **Google Analytics 4**: Analytics and event tracking (G-L162628GP4)
- **Content Source**: Notion database (synced via Python script)

## Essential Commands

### Development
```bash
# Start development server with Tailwind watch mode
./dev.sh

# Or manually:
# Terminal 1: Watch Tailwind CSS
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch

# Terminal 2: Start Zola server
zola serve

# Build site for production
./build.sh

# Or manually:
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify
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
│   ├── services.md      # Services page
│   └── contact.md       # Contact page
├── templates/           # Tera templates (HTML)
│   ├── base.html        # Base layout with GA4, search, dark mode
│   ├── index.html       # Homepage with featured posts
│   ├── section.html     # Blog listing with pagination
│   ├── page.html        # Individual posts with TOC, reactions, related posts
│   ├── 404.html         # Custom error page
│   ├── tags/            # Tag taxonomy templates
│   └── categories/      # Category taxonomy templates
├── static/              # Static assets (copied as-is)
│   ├── css/
│   │   └── tailwind.css # Compiled Tailwind CSS
│   ├── images/          # Featured images, OG image, hero graphics
│   ├── favicon.svg      # Site favicon
│   ├── _headers         # Cloudflare headers (cache, security)
│   ├── _redirects       # Cloudflare redirects
│   └── robots.txt       # SEO and AI bot blocking
├── styles/              # Tailwind CSS v4 source files
│   └── input.css        # Main Tailwind source
├── docs/                # Project documentation
├── build.sh             # Production build script
├── dev.sh               # Development script
├── sync.py              # Notion sync script
├── tailwindcss          # Tailwind CLI binary
└── public/              # Generated site (gitignored)
```

## Architecture Overview

### Template Hierarchy
- **base.html**: Root template with header, footer, navigation, search modal, dark mode toggle, and GA4 tracking
- **index.html**: Homepage with hero section, featured posts with images, and recent posts grid
- **section.html**: Blog listing pages with featured images, excerpts, and pagination
- **page.html**: Individual posts with:
  - Reading progress bar
  - Expandable table of contents
  - Post reactions (👍 ✨ ❤️ 🤯)
  - Reading list bookmark
  - Related posts widget
  - Social sharing buttons
  - FAQ schema for SEO
- **Taxonomy templates**: Tag and category pages with counts and descriptions

### Content Flow
1. Markdown files in `content/` define pages and posts
2. Frontmatter (TOML between `+++`) sets metadata (title, date, tags, categories)
3. Zola processes templates with Tera templating engine
4. Static site generated in `public/` directory

### Key Zola Features Used
- **Taxonomies**: Automatic tag and category pages with RSS feeds
- **Search**: Built-in elasticlunr.js search index with async loading
- **RSS Feeds**: Auto-generated for main blog and taxonomies
- **Syntax Highlighting**: Code blocks with `base16-ocean-dark` theme and copy button
- **Tailwind CSS**: Utility-first styling with custom design system
- **Performance**: Optimized with preconnect, cache headers, and async loading

## Content Management

### Blog Posts
Blog posts are stored in `content/blog/` with this frontmatter structure:

```toml
+++
title = "Post Title"
date = 2025-01-01
description = "Brief description for SEO"
featured_image = "/images/post-image.svg"  # Optional featured image

[taxonomies]
categories = ["Category Name"]
tags = ["tag1", "tag2"]

[extra]
author = "Hope"  # Optional author override
+++
```

### Static Pages
Pages like About and Services don't require a `date` field in frontmatter. The template handles optional dates gracefully.

## Notion Integration (Week 2 - Implemented)

The project uses a simple Python script (`sync.py`) that syncs published blog posts from Notion to Zola.

### Quick Start

1. **Setup** (first time only):
   ```bash
   # Install Python dependency
   pip3 install requests

   # Copy environment template
   cp .env.example .env
   # Edit .env with your NOTION_API_KEY and NOTION_DATABASE_ID
   ```

2. **Run sync**:
   ```bash
   # Load environment variables and run sync
   source .env
   python3 sync.py
   ```

3. **Build site** after sync:
   ```bash
   zola serve  # Preview
   zola build  # Production build
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
- Paragraphs with rich text formatting (bold, italic, code, strikethrough, links)
- Headings (H1, H2, H3)
- Bulleted and numbered lists
- Code blocks with syntax highlighting
- Quotes and callouts
- Images (external and uploaded)
- Dividers

See `SYNC_README.md` for detailed setup instructions and troubleshooting.

## Development Workflow

1. **Adding a blog post manually**: Create a new `.md` file in `content/blog/` with proper frontmatter, or use `python3 new_post.py`
2. **Syncing from Notion**: Run `source .env && python3 sync.py` to pull published posts
3. **Testing changes**: Run `./dev.sh` (or `zola serve` + Tailwind watch) and visit `http://127.0.0.1:1111`
4. **Building for production**: Run `./build.sh` to compile Tailwind CSS and build Zola site
5. **Template changes**: Modify files in `templates/` - changes auto-reload with `zola serve`
6. **Styling changes**: Edit `styles/input.css` - Tailwind CLI auto-rebuilds with watch mode
7. **Deploying**: Push to `main` branch - Cloudflare Pages auto-deploys

## Important Notes

- The `public/` directory is gitignored - it's regenerated on each build
- Zola uses Tera templating syntax (similar to Jinja2)
- Blog posts must have a `date` field for proper sorting
- Taxonomies (tags/categories) are automatically generated from frontmatter
- RSS feeds are auto-generated for blog and each taxonomy
- Site rebuilds are extremely fast (typically <100ms for small sites)

## Project Status

**Current State**: ✅ **LIVE IN PRODUCTION**
- **URL**: https://www.mileshope.com
- **Hosting**: Cloudflare Pages with auto-deploy from `main` branch
- **Analytics**: Google Analytics 4 (G-L162628GP4) tracking page views and custom events
- **Content**: 14+ blog posts synced from Notion, fully populated About/Services/Contact pages

**Completed Features**:

### Core Infrastructure (Weeks 1-2)
- ✅ Zola static site generator with Tailwind CSS v4
- ✅ Notion API integration with Python sync script
- ✅ Git repository with clean commit history
- ✅ Cloudflare Pages deployment pipeline
- ✅ Custom domain with SSL

### Design & Styling (Week 3)
- ✅ Purple/gold spiritual color scheme
- ✅ Lora (headings) + Inter (body) typography
- ✅ Dark mode with localStorage persistence
- ✅ Mobile responsive with hamburger menu
- ✅ Reading progress bar
- ✅ Custom SVG featured images for all posts

### Content & SEO (Week 4)
- ✅ 14+ comprehensive blog posts on bazi, tarot, and spirituality
- ✅ Fully populated About, Services, and Contact pages
- ✅ Open Graph and Twitter Card meta tags
- ✅ FAQ Schema for rich snippets (Google Rich Results eligible)
- ✅ Favicon and OG image
- ✅ robots.txt with AI bot blocking

### Advanced Features (Week 5+)
- ✅ **Search**: Client-side elasticlunr.js with async loading
- ✅ **Blog Features**:
  - Expandable table of contents
  - Post reactions (👍 ✨ ❤️ 🤯)
  - Reading list bookmarks
  - Related posts widget
  - Copy code button
  - Social sharing
- ✅ **Analytics**:
  - Scroll depth tracking
  - Active time on page
  - Exit intent detection
  - 14+ custom GA4 events
- ✅ **Performance**:
  - Async resource loading
  - Cache control headers
  - Security headers (CSP, X-Frame-Options, etc.)
  - Layout shift fixes
  - requestAnimationFrame optimizations

### Documentation
- ✅ `docs/DEPLOYMENT.md` - Deployment guide
- ✅ `docs/NOTION_SYNC.md` - Notion sync documentation
- ✅ `docs/TESTING.md` - Testing guide
- ✅ `docs/GA4_SETUP.md` - Analytics setup
- ✅ `docs/ARCHITECTURE.md` - Technical architecture
- ✅ Project README with quick start guide

**Metrics**:
- ~70+ commits with clean history
- 14+ live blog posts
- Fast builds (~50-100ms with Zola)
- Mobile responsive across all breakpoints
- WCAG 2.1 accessibility compliant
- Privacy-friendly (localStorage only, no cookies)
