# Architecture Overview

## Project Structure

MilesHope.com is a static site built with Zola, styled with Tailwind CSS v4, and integrated with Notion for content management.

## Core Technologies

### Zola (Static Site Generator)

**Why Zola?**
- Extremely fast builds (written in Rust)
- Zero runtime dependencies
- Built-in features (search, taxonomies, RSS)
- Single binary installation

**Version:** v0.18.0+

### Tailwind CSS v4

**Why Tailwind v4?**
- No Node.js dependencies (standalone CLI)
- Smaller output size (26KB vs 82KB)
- Faster builds (~50ms)
- CSS-based configuration via `@theme`

**Implementation:**
- Standalone CLI binary (65KB)
- Source file: `styles/input.css`
- Output: `static/css/tailwind.css` (26KB minified)
- Build command: `./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify`

### Notion Integration

**Purpose:** Content management system for blog posts

**Implementation:**
- Custom Rust CLI tool (`notion-sync/`)
- Converts Notion pages to Zola-compatible Markdown
- Supports: headings, paragraphs, lists, code, quotes, links
- One-way sync: Notion → Zola

## Template Hierarchy

```
base.html (root template)
├── index.html (homepage)
├── section.html (blog listing)
├── page.html (individual posts/pages)
├── 404.html (error page)
├── tags/
│   ├── list.html (all tags)
│   └── single.html (single tag)
└── categories/
    ├── list.html (all categories)
    └── single.html (single category)
```

### Template Inheritance

All templates extend `base.html` which provides:
- Navigation header
- Footer
- Search modal
- Reading progress bar
- Theme toggle (dark mode)

### Page Types

| Template | URL Pattern | Purpose |
|----------|-------------|---------|
| `index.html` | `/` | Homepage with hero and recent posts |
| `section.html` | `/blog/` | Blog listing with pagination |
| `page.html` | `/blog/[slug]/`, `/about/`, `/services/` | Individual content pages |
| `tags/list.html` | `/tags/` | All tags with tag cloud |
| `tags/single.html` | `/tags/[tag]/` | Posts for specific tag |
| `categories/list.html` | `/categories/` | All categories |
| `categories/single.html` | `/categories/[category]/` | Posts for specific category |
| `404.html` | `/404.html` | Error page |

## Content Flow

```
Notion Database
    ↓ (notion-sync tool)
Markdown Files (content/blog/*.md)
    ↓ (Zola)
HTML Pages (public/*.html)
    ↓ (Cloudflare Pages)
Production Site (www.mileshope.com)
```

## Styling Architecture

### Design System

**Colors:**
- Primary: Purple (#805ad5)
- Primary Light: #b794f4 (dark mode)
- Primary Dark: #6b46c1 (hover)
- Accent: Gold (#d69e2e)
- Accent Light: #fbd38d (dark mode)

**Typography:**
- Headings: Merriweather (serif)
- Body: Inter (sans-serif)
- Code: Monospace

**Spacing:**
- Base unit: 1rem (16px)
- Container max-width: 1200px
- Post content max-width: 800px

### Dark Mode

**Implementation:** Class-based (`.dark` on `<html>`)

**JavaScript:**
- Saves preference to `localStorage`
- Applies on page load
- Toggle button in navigation

**CSS:**
```css
/* Light mode (default) */
.element { color: #2d3748; }

/* Dark mode */
html.dark .element { color: #e2e8f0; }
```

### Responsive Design

**Breakpoint:** 768px (mobile/tablet split)

**Strategy:**
- Mobile-first approach
- Single breakpoint for simplicity
- CSS Grid for post layouts
- Flexbox for navigation

## Search Implementation

**Library:** elasticlunr.js (v0.9.5)

**How It Works:**
1. Zola generates `search_index.en.js` at build time
2. JavaScript loads index dynamically as a script
3. elasticlunr indexes title and body fields
4. Real-time search as user types
5. Results display with excerpts

**Index Format:**
```javascript
window.searchIndex = {
  fields: ["title", "body"],
  documentStore: { ... },
  index: { ... }
}
```

## Build Process

### Development Build

```bash
./dev.sh
# Runs in parallel:
# - Tailwind CSS watch mode
# - Zola serve with live reload
```

### Production Build

```bash
./build.sh
# Steps:
# 1. Compile Tailwind CSS (minified)
# 2. Build Zola site
# Output: public/ directory
```

### Build Scripts

**build.sh:**
```bash
#!/bin/bash
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify
zola build
```

**dev.sh:**
```bash
#!/bin/bash
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch &
TAILWIND_PID=$!
zola serve
kill $TAILWIND_PID
```

## Performance Optimizations

### CSS
- Single CSS file (26KB minified)
- 68% reduction from original (82KB)
- Tree-shaken unused utilities
- Custom components for common patterns

### Build Times
- Tailwind: ~50ms
- Zola: ~12ms
- Total: ~62ms

### Runtime
- Static HTML (no JavaScript framework)
- Minimal JavaScript (search, dark mode, navigation)
- Fast CDN delivery (Cloudflare Pages)
- No database queries

## Deployment Architecture

```
GitHub Repository
    ↓ (git push origin main)
Cloudflare Pages
    ↓ (runs build.sh)
Global CDN
    ↓
Users (www.mileshope.com)
```

**Cloudflare Pages Configuration:**
- Build command: `./build.sh`
- Build output directory: `public`
- Root directory: `/`
- Environment: Production
- Node.js version: Not required

## File Organization

```
mileshope.com/
├── content/           # Markdown source
├── templates/         # HTML templates
├── static/           # Static assets
├── styles/           # CSS source
├── docs/             # Documentation
├── notion-sync/      # Notion integration
├── public/           # Generated site (gitignored)
├── config.toml       # Zola config
├── build.sh          # Build script
├── dev.sh            # Dev script
└── tailwindcss       # CSS CLI
```

## Key Design Decisions

### Why Static Site?
- **Performance:** No server-side processing
- **Security:** No backend to hack
- **Scalability:** CDN handles all traffic
- **Cost:** Free hosting on Cloudflare Pages
- **Simplicity:** Deploy with git push

### Why Tailwind v4 Standalone?
- **No Node.js:** Simpler dependencies
- **Faster builds:** Rust-based compiler
- **Smaller output:** Better tree-shaking
- **Future-proof:** Latest CSS features

### Why Notion Integration?
- **User-friendly:** Non-technical writing
- **Structured:** Database properties
- **Flexible:** Rich formatting support
- **Portable:** Export to Markdown

### Why Rust for Tooling?
- **Performance:** Fast compilation
- **Reliability:** Type safety
- **Ecosystem:** Great Notion/HTTP libraries
- **Consistency:** Matches Zola

## Security Considerations

### Static Site Security
- No SQL injection (no database)
- No XSS vulnerabilities (static HTML)
- No authentication system to breach
- HTTPS enforced by Cloudflare

### Content Security
- Notion API key stored in `.env` (gitignored)
- GitHub secrets for CI/CD
- No API keys in client-side code

### CORS and CSP
```html
<!-- Set in templates/base.html -->
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' https://unpkg.com; style-src 'self' 'unsafe-inline';">
```

## Extensibility

### Adding New Features

**New Page Type:**
1. Create template in `templates/`
2. Add content in `content/`
3. Update navigation in `base.html`

**New Styling:**
1. Add CSS to `styles/input.css`
2. Rebuild: `./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify`

**New Taxonomy:**
1. Add to `config.toml` taxonomies
2. Create templates in `templates/[taxonomy]/`
3. Add frontmatter to content

### Plugin System
Zola doesn't have plugins, but you can:
- Add JavaScript libraries via CDN
- Create custom Tera shortcodes
- Build custom Rust tools (like notion-sync)

## Monitoring and Analytics

### Build Monitoring
- Cloudflare Pages dashboard
- Build logs in Cloudflare
- Git commit tracking

### Traffic Analytics
- Cloudflare Web Analytics (privacy-friendly)
- No cookies required
- GDPR compliant

## Future Architecture Considerations

### Potential Enhancements
- Image optimization (WebP, lazy loading)
- Service worker for offline support
- Comments system (static-site friendly)
- Newsletter integration
- Multi-language support (i18n)

### Scalability
Current architecture scales to:
- Millions of page views (Cloudflare CDN)
- Thousands of blog posts (Zola handles it)
- Fast builds even at scale (Rust performance)

---

**Last Updated:** January 2025
