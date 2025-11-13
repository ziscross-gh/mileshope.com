# Development Guide

Complete guide for developing MilesHope.com locally.

## Prerequisites

### Required Software

1. **Zola** (v0.18.0+)
   ```bash
   # macOS
   brew install zola

   # Linux
   # Download from https://github.com/getzola/zola/releases

   # Windows
   # Download from https://github.com/getzola/zola/releases
   ```

2. **Rust & Cargo** (for Notion sync tool)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Git**
   ```bash
   # macOS
   brew install git

   # Linux
   sudo apt-get install git
   ```

### Optional Tools

- **VS Code** with extensions:
  - Zola syntax highlighting
  - Tailwind CSS IntelliSense
  - Rust Analyzer (for notion-sync)

## Initial Setup

### 1. Clone Repository

```bash
git clone https://github.com/ziscross-gh/mileshope.com.git
cd mileshope.com
```

### 2. Verify Tailwind CSS CLI

The Tailwind CSS v4 standalone CLI should already be in the repo:

```bash
ls -lh tailwindcss
# Should show: -rwxr-xr-x  1 user  staff   65K  tailwindcss
```

If missing, download it:
```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x tailwindcss-macos-arm64
mv tailwindcss-macos-arm64 tailwindcss
```

### 3. Make Scripts Executable

```bash
chmod +x build.sh dev.sh
```

## Development Workflow

### Starting Development Server

**Option 1: Automated (Recommended)**
```bash
./dev.sh
```

This runs both:
- Tailwind CSS in watch mode (auto-recompiles on changes)
- Zola serve with live reload

**Option 2: Manual (Two terminals)**

Terminal 1:
```bash
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch
```

Terminal 2:
```bash
zola serve
```

### Accessing the Site

Open http://127.0.0.1:1111 in your browser.

Live reload is enabled:
- Template changes → instant reload
- Content changes → instant reload
- CSS changes → instant reload (watch mode)

### Stopping Development Server

Press `Ctrl+C` in the terminal running `dev.sh` or `zola serve`.

## Making Changes

### Editing Content

#### Blog Posts

**Location:** `content/blog/`

**Format:**
```markdown
+++
title = "Post Title"
date = 2025-01-13
description = "Brief description for SEO"

[taxonomies]
categories = ["Category Name"]
tags = ["tag1", "tag2"]
+++

Your content here...
```

**Creating New Post:**
```bash
# Manual
touch content/blog/new-post.md
# Edit with your editor

# Or sync from Notion (see NOTION_SYNC.md)
cd notion-sync
cargo run
```

#### Static Pages

**About Page:** `content/about.md`
**Services Page:** `content/services.md`

Edit directly in your text editor.

### Editing Templates

**Location:** `templates/`

**Common files:**
- `base.html` - Header, footer, navigation (all pages use this)
- `index.html` - Homepage layout
- `section.html` - Blog listing page
- `page.html` - Individual post/page layout

**Template Syntax:** [Tera](https://keats.github.io/tera/) (similar to Jinja2)

**Example:**
```html
{% block content %}
<div class="container">
  {{ section.content | safe }}
</div>
{% endblock %}
```

### Editing Styles

**Location:** `styles/input.css`

**Structure:**
```css
@import "tailwindcss";

@theme {
  /* Custom theme config */
}

/* Custom components */
.my-component {
  /* styles */
}
```

**After editing:**
- Watch mode automatically rebuilds
- Or manually: `./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css`

### Configuration

**Location:** `config.toml`

**Common settings:**
```toml
base_url = "https://www.mileshope.com"
title = "MilesHope.com"
description = "..."

[extra]
author = "Miles Hope"
```

## Testing Changes

### Local Testing Checklist

- [ ] Homepage loads correctly
- [ ] Blog listing shows posts
- [ ] Individual post pages work
- [ ] Tags page displays all tags
- [ ] Categories page displays all categories
- [ ] Search functionality works
- [ ] Dark mode toggle works
- [ ] Mobile responsive layout
- [ ] Navigation menu (desktop + mobile)
- [ ] All links work

### Browser Testing

Test in multiple browsers:
- Chrome/Brave
- Firefox
- Safari
- Mobile browsers (Chrome Mobile, Safari iOS)

### Responsive Testing

Test at these widths:
- 375px (mobile)
- 768px (tablet)
- 1024px (small desktop)
- 1440px (large desktop)

Use browser DevTools device emulation.

## Building for Production

### Test Production Build

```bash
./build.sh
```

Output in `public/` directory.

### Serve Production Build Locally

```bash
cd public
python3 -m http.server 8000
```

Visit http://localhost:8000

### Check Build Output

```bash
# File sizes
du -sh public/*

# CSS size
ls -lh public/css/tailwind.css

# Total site size
du -sh public
```

## Common Tasks

### Adding a New Page

1. Create markdown file:
   ```bash
   touch content/my-new-page.md
   ```

2. Add frontmatter:
   ```markdown
   +++
   title = "My New Page"
   +++

   Content here...
   ```

3. View at http://127.0.0.1:1111/my-new-page/

### Adding a New Component

1. Add CSS to `styles/input.css`:
   ```css
   .my-component {
     padding: 1rem;
     background: #f7fafc;
   }
   ```

2. Use in template:
   ```html
   <div class="my-component">
     Content
   </div>
   ```

3. Tailwind rebuilds automatically (watch mode)

### Adding Dark Mode Styles

```css
/* Light mode */
.element {
  color: #2d3748;
}

/* Dark mode */
html.dark .element {
  color: #e2e8f0;
}
```

### Updating Navigation

Edit `templates/base.html`:

```html
<ul class="nav-menu" id="navMenu">
  <li><a href="{{ get_url(path='@/blog/_index.md') }}">Blog</a></li>
  <li><a href="{{ get_url(path='@/about.md') }}">About</a></li>
  <li><a href="{{ get_url(path='@/my-new-page.md') }}">New Page</a></li>
  <!-- ... -->
</ul>
```

## Troubleshooting

### Zola Won't Start

**Error:** `Address already in use`
```bash
# Find and kill the process using port 1111
lsof -ti:1111 | xargs kill -9
```

### Tailwind CSS Not Rebuilding

1. Stop watch mode (`Ctrl+C`)
2. Manually rebuild:
   ```bash
   ./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css
   ```
3. Restart watch mode

### Changes Not Showing

1. Hard refresh browser (`Cmd+Shift+R` or `Ctrl+Shift+R`)
2. Clear browser cache
3. Check browser console for errors
4. Verify file saved correctly

### Search Not Working

1. Rebuild to regenerate search index:
   ```bash
   zola build
   ```
2. Check `public/search_index.en.js` exists
3. Check browser console for errors

### Dark Mode Not Working

1. Check `localStorage` in browser DevTools:
   ```javascript
   localStorage.getItem('theme')
   ```
2. Verify `<html>` has `dark` class
3. Check CSS dark mode styles exist

## Git Workflow

### Creating a Feature Branch

```bash
git checkout -b feature/my-feature
# Make changes
git add .
git commit -m "Add my feature"
git push origin feature/my-feature
```

### Syncing with Main

```bash
git checkout main
git pull origin main
git checkout feature/my-feature
git merge main
```

### Committing Changes

```bash
# Stage specific files
git add templates/base.html styles/input.css

# Or stage all
git add .

# Commit with clear message
git commit -m "Add new navigation item

- Added Services link to main navigation
- Updated mobile menu to include Services
- Tested on desktop and mobile viewports"

# Push
git push origin feature/my-feature
```

## Performance Tips

### Optimizing CSS

- Remove unused custom components
- Use Tailwind utilities where possible
- Minimize custom CSS

### Optimizing Images

```bash
# Convert to WebP
cwebp input.png -o output.webp

# Optimize JPEG
jpegoptim --max=85 image.jpg
```

### Optimizing Builds

- Keep `public/` out of git (it's in `.gitignore`)
- Use `--minify` flag for production CSS
- Zola is already optimized (Rust)

## IDE Setup

### VS Code

**Recommended extensions:**
```json
{
  "recommendations": [
    "bungcip.better-toml",
    "tailwindlabs.tailwind-css-intellisense",
    "rust-lang.rust-analyzer",
    "esbenp.prettier-vscode"
  ]
}
```

**Workspace settings:**
```json
{
  "files.associations": {
    "*.html": "html"
  },
  "tailwindCSS.includeLanguages": {
    "html": "html"
  },
  "tailwindCSS.experimental.classRegex": [
    "class=\"([^\"]*)\""
  ]
}
```

## Resources

### Documentation

- [Zola Docs](https://www.getzola.org/documentation/)
- [Tera Template Docs](https://keats.github.io/tera/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/docs)

### Community

- [Zola Discourse](https://zola.discourse.group/)
- [Tailwind Discord](https://tailwindcss.com/discord)

### Tools

- [Can I Use](https://caniuse.com/) - Browser compatibility
- [Lighthouse](https://developers.google.com/web/tools/lighthouse) - Performance testing
- [WebPageTest](https://www.webpagetest.org/) - Speed testing

---

**Last Updated:** January 2025
