# MilesHope.com

A static blog website exploring the intersection of spirituality, technology, and personal growth.

## Tech Stack

- **Zola**: Fast static site generator written in Rust
- **Deployment**: Cloudflare Pages (planned)
- **Content Source**: Notion database (integration in progress)

## Quick Start

### Prerequisites

- [Zola](https://www.getzola.org/documentation/getting-started/installation/) installed

### Development

```bash
# Start development server (with live reload)
zola serve

# Build for production
zola build

# Check for errors
zola check
```

Visit `http://127.0.0.1:1111` to see the site locally.

## Project Structure

- `content/`: Markdown content files (blog posts, pages)
- `templates/`: Tera HTML templates
- `static/`: Static assets (CSS, JS, images)
- `config.toml`: Zola configuration
- `public/`: Generated site (gitignored)

## Features

- Fast static site generation
- RSS feeds for blog and taxonomies
- Built-in search functionality
- Tag and category support
- Syntax highlighting for code blocks
- Mobile-responsive design
- SEO optimized

## Roadmap

### Week 1: Foundation ✅
- [x] Install Zola and create project structure
- [x] Configure Zola with site settings
- [x] Create basic template structure
- [x] Set up GitHub repository

### Week 2: Notion Integration (Current)
- [ ] Build Rust CLI tool for Notion API
- [ ] Implement Notion-to-Markdown converter
- [ ] Create automated sync script

### Week 3: Design & Styling
- [ ] Enhance design with custom styling
- [ ] Implement responsive design improvements
- [ ] Add animations and visual polish

### Week 4: SEO & Features
- [ ] Optimize SEO meta tags
- [ ] Enhance search functionality
- [ ] Add social sharing features

### Week 5: Deployment
- [ ] Set up Cloudflare Pages
- [ ] Configure domain and SSL
- [ ] Automate deployment pipeline

### Week 6: Testing & Launch
- [ ] Cross-browser testing
- [ ] Performance optimization
- [ ] Final polish and launch

## Documentation

See [CLAUDE.md](CLAUDE.md) for detailed development documentation.

## License

Copyright © 2025 Miles Hope. All rights reserved.
