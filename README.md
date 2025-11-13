# MilesHope.com

A modern static blog website exploring the intersection of spirituality, technology, and personal growth. Built with Zola and styled with Tailwind CSS v4.

## 🚀 Tech Stack

- **Zola** - Fast static site generator written in Rust
- **Tailwind CSS v4** - Modern utility-first CSS framework (standalone CLI)
- **Notion** - Content management via Notion API
- **Cloudflare Pages** - Deployment platform
- **Rust** - Custom CLI tools for automation

## ✨ Features

- **Modern Design** - Clean, responsive design with Tailwind CSS v4
- **Dark Mode** - Full dark mode support with smooth transitions
- **Search** - Fast client-side search powered by elasticlunr.js
- **Blog** - Full-featured blog with posts, tags, and categories
- **RSS Feeds** - Auto-generated feeds for blog and taxonomies
- **SEO Optimized** - Meta tags, Open Graph, and Twitter Cards
- **Mobile Responsive** - Works perfectly on all screen sizes
- **Notion Sync** - Automated blog post syncing from Notion database
- **Performance** - 26KB CSS, fast builds (~50ms), optimized assets

## 🏁 Quick Start

### Prerequisites

- [Zola](https://www.getzola.org/documentation/getting-started/installation/) (v0.18.0+)
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (for Notion sync tool)

### Development

```bash
# Clone the repository
git clone https://github.com/ziscross-gh/mileshope.com.git
cd mileshope.com

# Start development server with Tailwind watch mode
./dev.sh

# Or manually start each process:
# Terminal 1: Watch and compile Tailwind CSS
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch

# Terminal 2: Start Zola server
zola serve
```

Visit `http://127.0.0.1:1111` to see the site locally.

### Production Build

```bash
# Build everything (Tailwind + Zola)
./build.sh

# Or manually:
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify
zola build
```

The production site will be in the `public/` directory.

## 📁 Project Structure

```
mileshope.com/
├── content/              # Markdown content
│   ├── blog/            # Blog posts (synced from Notion)
│   ├── about.md         # About page
│   └── services.md      # Services page
├── templates/           # Tera HTML templates
│   ├── base.html        # Base layout
│   ├── index.html       # Homepage
│   ├── section.html     # Blog listing
│   ├── page.html        # Individual posts
│   ├── tags/            # Tag taxonomy templates
│   └── categories/      # Category taxonomy templates
├── static/              # Static assets
│   └── css/
│       └── tailwind.css # Compiled Tailwind CSS (26KB)
├── styles/              # Tailwind source
│   └── input.css        # Tailwind v4 source file
├── notion-sync/         # Notion integration tool (Rust)
├── docs/                # Documentation
├── config.toml          # Zola configuration
├── build.sh             # Production build script
├── dev.sh               # Development script
└── tailwindcss          # Tailwind CSS v4 CLI (65KB)
```

## 📝 Documentation

All documentation is organized in the `docs/` directory:

- **[Architecture Overview](docs/ARCHITECTURE.md)** - Project structure and design decisions
- **[Development Guide](docs/DEVELOPMENT.md)** - Local development workflow
- **[Notion Sync Guide](docs/NOTION_SYNC.md)** - Using the Notion integration
- **[Tailwind Migration](docs/TAILWIND_MIGRATION.md)** - CSS migration history
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Cloudflare Pages setup

### For Claude Code

See [CLAUDE.md](CLAUDE.md) for AI assistant development instructions.

## 🔄 Notion Integration

Sync blog posts from Notion to Zola:

```bash
cd notion-sync
cp .env.example .env
# Edit .env with your NOTION_API_KEY and NOTION_DATABASE_ID
cargo run
```

See [docs/NOTION_SYNC.md](docs/NOTION_SYNC.md) for detailed setup.

## 🎨 Styling

This project uses **Tailwind CSS v4** with a custom design system:

- **Colors**: Purple (#805ad5) primary, Gold (#d69e2e) accent
- **Typography**: Serif headings (Merriweather), Sans body (Inter)
- **Dark Mode**: Class-based (`.dark` on `<html>`)
- **Breakpoint**: Single mobile breakpoint at 768px

To modify styles, edit `styles/input.css` and rebuild with `./tailwindcss`.

## 🚢 Deployment

The site automatically deploys to Cloudflare Pages when pushed to `main`:

1. Push to GitHub: `git push origin main`
2. Cloudflare Pages runs `build.sh`
3. Site deploys to https://www.mileshope.com

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for manual deployment steps.

## 📊 Performance

- **CSS Size**: 26KB (68% reduction from 82KB original)
- **Build Time**: ~50ms (Tailwind) + 12ms (Zola) = 62ms total
- **Lighthouse**: 100/100 (Performance, Accessibility, Best Practices, SEO)

## 🗺️ Roadmap

### ✅ Completed

- [x] **Week 1**: Zola setup and basic templates
- [x] **Week 2**: Notion integration with Rust CLI
- [x] **Week 3**: Navigation, layout, and dark mode
- [x] **Week 4**: Content components and styling
- [x] **Week 5**: Search functionality and interactive features
- [x] **Week 6**: Tailwind CSS v4 migration complete

### 🔮 Future Enhancements

- [ ] Newsletter subscription
- [ ] Comment system
- [ ] Image optimization and lazy loading
- [ ] Advanced analytics
- [ ] Multi-language support

## 🤝 Contributing

This is a personal project, but suggestions and feedback are welcome! Feel free to open an issue.

## 📄 License

Copyright © 2025 Miles Hope. All rights reserved.

---

**Built with ❤️ using Zola, Tailwind CSS v4, and Rust**
