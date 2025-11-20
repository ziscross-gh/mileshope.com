# MilesHope.com

A modern static blog exploring spirituality, technology, and personal growth. Built with Zola and styled with Tailwind CSS v4.

**Live site**: https://www.mileshope.com

## Tech Stack

- **Zola** - Fast static site generator written in Rust
- **Tailwind CSS v4** - Utility-first CSS framework (standalone CLI)
- **Notion** - Content management via Notion API
- **Cloudflare Pages** - Deployment and hosting
- **Google Analytics 4** - Analytics and event tracking

## Features

- **Modern Design** - Purple/gold color scheme with Lora serif headings
- **Dark Mode** - Full dark mode support with smooth transitions
- **Search** - Client-side search powered by elasticlunr.js
- **Blog** - Posts with tags, categories, reading time, and table of contents
- **Related Posts** - Smart post recommendations based on tags
- **RSS Feeds** - Auto-generated feeds for blog and taxonomies
- **SEO Optimized** - Meta tags, Open Graph, Twitter Cards, structured data
- **Mobile Responsive** - Hamburger menu and responsive layouts
- **Notion Sync** - Automated blog post syncing from Notion database
- **Analytics** - GA4 with custom events (search, share, code copy)
- **Performance** - Fast builds (~50ms), optimized CSS

## Quick Start

### Prerequisites

- [Zola](https://www.getzola.org/documentation/getting-started/installation/) (v0.18.0+)
- Python 3 (for Notion sync)

### Development

```bash
# Clone the repository
git clone https://github.com/ziscross-gh/mileshope.com.git
cd mileshope.com

# Start development server with Tailwind watch mode
./dev.sh

# Or manually:
# Terminal 1: Watch Tailwind CSS
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch

# Terminal 2: Start Zola server
zola serve
```

Visit `http://127.0.0.1:1111` to see the site locally.

### Production Build

```bash
# Build everything
./build.sh

# Or manually:
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify
zola build
```

## Project Structure

```
mileshope.com/
├── content/              # Markdown content
│   ├── blog/            # Blog posts
│   ├── about.md         # About page
│   ├── contact.md       # Contact page
│   └── services.md      # Services page
├── templates/           # Tera HTML templates
│   ├── base.html        # Base layout with GA4
│   ├── index.html       # Homepage
│   ├── section.html     # Blog listing
│   ├── page.html        # Individual posts
│   └── 404.html         # Error page
├── static/              # Static assets
│   ├── css/tailwind.css # Compiled CSS
│   ├── favicon.svg      # Site favicon
│   └── images/          # Images and OG image
├── styles/input.css     # Tailwind v4 source
├── config.toml          # Zola + GA4 configuration
├── sync.py              # Notion sync script
├── build.sh             # Production build
└── dev.sh               # Development script
```

## Notion Sync

Sync blog posts from Notion:

```bash
# Setup
pip3 install requests
cp .env.example .env
# Edit .env with NOTION_API_KEY and NOTION_DATABASE_ID

# Sync
source .env
python3 sync.py
```

See [SYNC_README.md](SYNC_README.md) for detailed setup.

## Styling

Custom design system with Tailwind CSS v4:

- **Colors**: Purple (#805ad5) primary, Gold (#d69e2e) accent
- **Typography**: Lora (headings), Inter (body)
- **Dark Mode**: Class-based toggle with localStorage persistence

Edit `styles/input.css` and rebuild with `./tailwindcss`.

## Deployment

Auto-deploys to Cloudflare Pages on push to `main`:

1. Push: `git push origin main`
2. Cloudflare runs `build.sh`
3. Deploys to https://www.mileshope.com

## Analytics

Google Analytics 4 is configured with custom event tracking:
- Page views
- Search queries
- Share button clicks
- Code block copies
- Navigation clicks

## License

Copyright 2025 Miles Hope. All rights reserved.
