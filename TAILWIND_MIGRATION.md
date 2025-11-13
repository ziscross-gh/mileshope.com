# Tailwind CSS Migration Progress

## Overview
Incremental migration from custom CSS to Tailwind CSS v4 for MilesHope.com

**Strategy:** Hybrid CSS approach (both files loaded during migration)
**Timeline:** 6 weeks
**Branch:** `tailwind-migration`

---

## Week 1: Infrastructure Setup ✅ COMPLETE

### Completed Tasks
- ✅ Downloaded Tailwind CSS v4 standalone CLI (65KB)
- ✅ Created `styles/input.css` with custom base styles
- ✅ Updated `build.sh` to compile Tailwind before Zola
- ✅ Created `dev.sh` for local development (watch mode)
- ✅ Updated `templates/base.html` to load both CSS files
- ✅ Tested build process (works locally and for Cloudflare Pages)
- ✅ Created `tailwind-migration` git branch
- ✅ Committed infrastructure setup

### Configuration
**Tailwind CSS:**
- Version: v4.1.17 (standalone CLI)
- No config file needed (v4 uses CSS-based config)
- Custom components defined in `styles/input.css`
- Minified output: 65KB (will shrink as we optimize)

**Build Process:**
```bash
# Development (watch mode for both)
./dev.sh

# Production
./build.sh  # Compiles Tailwind + Zola
```

### Files Created
- `styles/input.css` - Tailwind entry point
- `static/css/tailwind.css` - Generated output
- `dev.sh` - Development script

### Files Modified
- `build.sh` - Added Tailwind compilation
- `templates/base.html` - Load both CSS files
- `.gitignore` - Exclude Tailwind binary

### Custom Components Added
```css
.reading-progress  - Purple-to-gold gradient progress bar
.hero-gradient     - Purple gradient background
.avatar-gradient   - Purple-to-gold avatar background
```

### Next Steps
Continue to Week 2: Typography & Base Styles

---

## Week 2: Typography & Base Styles (In Progress)

### Planned Tasks
- [ ] Configure custom font families in CSS
- [ ] Migrate heading styles (h1-h6)
- [ ] Migrate paragraph and link styles
- [ ] Update dark mode to use `.dark` class
- [ ] Update theme toggle JavaScript
- [ ] Migrate text color utilities
- [ ] Test typography across all pages

### Success Criteria
- All text uses Tailwind typography classes
- Dark mode working with `.dark` class
- Theme toggle persists preference
- No visual regressions

---

## Week 3: Layout & Navigation (Planned)

### Tasks
- [ ] Migrate header/navbar
- [ ] Migrate mobile menu
- [ ] Migrate footer
- [ ] Migrate container/spacing
- [ ] Improve responsive breakpoints
- [ ] Optimize animations

---

## Week 4: Content Components (Planned)

### Tasks
- [ ] Migrate post cards
- [ ] Migrate post listings
- [ ] Migrate single post layout
- [ ] Apply @tailwindcss/typography plugin
- [ ] Migrate author bio
- [ ] Migrate tag pills

---

## Week 5: Interactive Components (Planned)

### Tasks
- [ ] Migrate buttons
- [ ] Migrate search modal
- [ ] Migrate reading progress bar
- [ ] Migrate social share buttons
- [ ] Test all JavaScript interactions

---

## Week 6: Polish & Cleanup (Planned)

### Tasks
- [ ] Migrate taxonomy pages
- [ ] Migrate 404 page
- [ ] Performance audit
- [ ] Remove old `style.css`
- [ ] Update documentation
- [ ] Merge to `main`
- [ ] Deploy to production

---

## Technical Notes

### Tailwind v4 Changes
Tailwind v4 is a major rewrite with significant changes:

1. **No config file** - Configuration via CSS `@theme` directive
2. **New syntax** - `@import "tailwindcss"` instead of `@tailwind`
3. **Faster builds** - Rust-based compiler
4. **Smaller output** - Better tree-shaking

### Custom Color Palette
```css
/* Spiritual colors preserved from original design */
Purple: #805ad5 (primary)
Purple Light: #b794f4 (dark mode)
Purple Dark: #6b46c1 (hover)
Gold: #d69e2e (accent)
Gold Light: #fbd38d (dark mode)
```

### Dark Mode Strategy
- Use `.dark` class on `<html>` element
- JavaScript toggles class and saves to localStorage
- Tailwind utilities: `dark:text-white`, `dark:bg-gray-800`, etc.

### Performance Goals
- **Current CSS:** 17KB (style.css)
- **Target with Tailwind:** ~15-20KB (after optimization)
- **Build time:** < 500ms (currently ~170ms for Tailwind)

---

## Development Workflow

### Starting Development
```bash
# Start both Tailwind watch and Zola serve
./dev.sh

# Or manually in separate terminals:
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch
zola serve
```

### Making Changes
1. Edit templates with Tailwind classes
2. Tailwind auto-rebuilds (watch mode)
3. Zola auto-reloads (serves at http://127.0.0.1:1111)
4. Test in browser
5. Commit changes to `tailwind-migration` branch

### Building for Production
```bash
./build.sh  # Runs Tailwind + Zola builds
```

---

## Migration Checklist

### Phase 1: Setup ✅
- [x] Install Tailwind CLI
- [x] Create build scripts
- [x] Update templates to load both CSS
- [x] Test build process

### Phase 2: Base Styles (Week 2)
- [ ] Typography
- [ ] Colors
- [ ] Dark mode
- [ ] Spacing

### Phase 3: Layout (Week 3)
- [ ] Header
- [ ] Navigation
- [ ] Footer
- [ ] Grid/Container

### Phase 4: Components (Week 4-5)
- [ ] Post cards
- [ ] Buttons
- [ ] Forms
- [ ] Modals
- [ ] Tags

### Phase 5: Cleanup (Week 6)
- [ ] Remove old CSS
- [ ] Optimize output
- [ ] Documentation
- [ ] Deploy

---

## Rollback Plan

If issues arise:
1. **Quick rollback:** Remove Tailwind CSS link from `base.html`
2. **Full rollback:** `git checkout main` (old CSS still works)
3. **Partial rollback:** Keep both CSS files, debug incrementally

---

## Resources

- **Tailwind v4 Docs:** https://tailwindcss.com/blog/tailwindcss-v4-beta
- **Migration Guide:** https://tailwindcss.com/docs/upgrade-guide
- **Zola Docs:** https://www.getzola.org/documentation/

---

## Progress Tracking

**Overall Progress:** 1/6 weeks (16%)

- ✅ Week 1: Infrastructure (100%)
- ⏳ Week 2: Typography (0%)
- ⏳ Week 3: Layout (0%)
- ⏳ Week 4: Content (0%)
- ⏳ Week 5: Interactive (0%)
- ⏳ Week 6: Cleanup (0%)

**Last Updated:** November 13, 2025
**Current Branch:** `tailwind-migration`
**Status:** Week 1 Complete, Ready for Week 2

---

*Migration managed with Claude Code*
