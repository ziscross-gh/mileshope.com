# Complete Pages Checklist for MilesHope.com

This checklist ensures all pages and their styles are accounted for during any CSS migration or major styling changes.

## Page Types & Templates

### ✅ 1. Homepage (`templates/index.html`)
**URL:** `/`

**Components:**
- `.homepage` - Main wrapper
- `.hero` - Hero section with gradient
- `.hero-title` - Large heading
- `.hero-subtitle` - Subtitle text
- `.hero-cta` - CTA button container
- `.btn-primary` - Primary button
- `.btn-secondary` - Secondary button
- `.recent-posts` - Recent posts section
- `.post-grid` - Responsive grid (auto-fill)
- `.post-card` - Individual post cards with hover
- `.post-meta` - Date and reading time
- `.post-tags` - Tag pills
- `.view-all` - View all posts link

**Status:** ✅ Complete (Week 4)

---

### ✅ 2. Blog Listing (`templates/section.html`)
**URL:** `/blog/`

**Components:**
- `.blog-section` - Main wrapper
- `.section-header` - Page header (centered)
- `.section-description` - Optional description
- `.posts-list` - Posts container (max-width 800px)
- `.post-item` - Individual post item
- `.post-item h2 a` - Post title links
- `.post-excerpt` - Post summary
- `.post-meta` - Date and reading time
- `.post-categories` - Category pills
- `.post-tags` - Tag pills
- `.pagination` - Previous/Next navigation
- `.pagination-link` - Pagination buttons
- `.pagination-info` - Page number display

**Status:** ✅ Complete (Added after Week 6)

---

### ✅ 3. Individual Blog Posts (`templates/page.html`)
**URL:** `/blog/[slug]/`

**Components:**
- `.blog-post` - Main wrapper
- `.blog-post .container` - Narrow container (800px)
- `.post-header` - Post header with border
- `.post-title` - Large post title (3rem)
- `.post-meta` - Date and reading time
- `.post-categories` - Category pills
- `.post-tags` - Tag pills
- `.post-content` - Rich content area
  - `.post-content h1/h2/h3` - Content headings
  - `.post-content p` - Paragraphs
  - `.post-content ul/ol` - Lists
  - `.post-content code` - Inline code
  - `.post-content pre` - Code blocks
  - `.post-content blockquote` - Quotes
  - `.post-content a` - Links
  - `.post-content img` - Images
- `.table-of-contents` - TOC sidebar
- `.author-bio` - Author bio box
- `.author-bio-content` - Bio flexbox
- `.author-avatar` - Avatar container
- `.avatar-placeholder` - Gradient avatar
- `.author-name` - Author name
- `.author-description` - Author bio text
- `.author-link` - Link to about page
- `.post-footer` - Footer section
- `.share-buttons` - Social sharing section
- `.share-btn-container` - Share buttons flexbox
- `.share-btn` - Share button base
- `.share-btn.twitter` - Twitter button
- `.share-btn.facebook` - Facebook button
- `.share-btn.linkedin` - LinkedIn button
- `.share-btn.copy` - Copy link button
- `.post-navigation` - Previous/Next navigation
- `.nav-previous` - Previous post link
- `.nav-next` - Next post link
- `.nav-label` - Navigation label
- `.nav-title` - Navigation title

**Status:** ✅ Complete (Added after Week 6)

---

### ✅ 4. Tags List (`templates/tags/list.html`)
**URL:** `/tags/`

**Components:**
- `.taxonomy-list` - Main wrapper
- `.taxonomy-header` - Page header (centered)
- `.tag-cloud` - Tag cloud flexbox
- `.tag-cloud .tag` - Individual tags
- `.tag-1` through `.tag-10` - Size variations

**Status:** ✅ Complete (Added after Week 6)

---

### ✅ 5. Single Tag (`templates/tags/single.html`)
**URL:** `/tags/[tag-name]/`

**Components:**
- `.taxonomy-page` - Main wrapper
- `.taxonomy-header` - Page header
- `.taxonomy-count` - Post count
- `.posts-list` - Posts container (reuses from section.html)
- `.post-item` - Individual posts (reuses)
- `.back-link` - Back to all tags link

**Status:** ✅ Complete (Added after Week 6)

---

### ✅ 6. Categories List (`templates/categories/list.html`)
**URL:** `/categories/`

**Components:**
- `.taxonomy-list` - Main wrapper (same as tags)
- `.taxonomy-header` - Page header
- `.tag-cloud` - Category cloud (reuses tag-cloud styles)

**Status:** ✅ Complete (Uses same styles as tags)

---

### ✅ 7. Single Category (`templates/categories/single.html`)
**URL:** `/categories/[category-name]/`

**Components:**
- `.taxonomy-page` - Main wrapper (same as tags)
- `.taxonomy-header` - Page header
- `.taxonomy-count` - Post count
- `.posts-list` - Posts container
- `.back-link` - Back to all categories link

**Status:** ✅ Complete (Uses same styles as tags)

---

### ✅ 8. About Page (`content/about.md` → `templates/page.html`)
**URL:** `/about/`

**Uses:** Same `.blog-post` styles as individual posts (without some components like author bio)

**Status:** ✅ Complete (Uses page.html template)

---

### ✅ 9. Services Page (`content/services.md` → `templates/page.html`)
**URL:** `/services/`

**Uses:** Same `.blog-post` styles as individual posts

**Status:** ✅ Complete (Uses page.html template)

---

### ✅ 10. 404 Page (`templates/404.html`)
**URL:** `/404.html`

**Components:**
- Standard layout (minimal styling, uses base typography)

**Status:** ✅ Complete (Uses base styles)

---

## Global Components

### ✅ Navigation (`templates/base.html`)
**Components:**
- `.site-header` - Sticky header
- `.navbar` - Navigation bar
- `.site-logo` - Site title/logo link
- `.mobile-menu-toggle` - Hamburger button
- `.hamburger` - Hamburger lines (3)
- `.nav-menu` - Navigation menu
- `.search-toggle` - Search button
- `.theme-toggle` - Dark mode button

**Status:** ✅ Complete (Week 3)

---

### ✅ Search Modal (`templates/base.html`)
**Components:**
- `.search-modal` - Modal overlay
- `.search-modal-content` - Modal content box
- `.search-header` - Header with input and close
- `#searchInput` - Search input field
- `.search-close` - Close button
- `.search-results` - Results container
- `.search-result-item` - Individual result
- `.search-result-excerpt` - Result excerpt
- `.search-no-results` - No results message

**Status:** ✅ Complete (Week 5)

---

### ✅ Footer (`templates/base.html`)
**Components:**
- `.site-footer` - Footer with background
- Footer text and links (uses base typography)

**Status:** ✅ Complete (Week 3)

---

### ✅ Reading Progress Bar (`templates/base.html`)
**Components:**
- `.reading-progress` - Gradient progress bar
- Custom gradient styles

**Status:** ✅ Complete (Week 1)

---

## CSS Migration Checklist

When migrating CSS or making major styling changes, verify ALL these pages:

### Testing Checklist
- [ ] Homepage - `/`
- [ ] Blog listing - `/blog/`
- [ ] Individual blog post - `/blog/[any-post]/`
- [ ] All tags page - `/tags/`
- [ ] Single tag page - `/tags/[any-tag]/`
- [ ] All categories page - `/categories/`
- [ ] Single category page - `/categories/[any-category]/`
- [ ] About page - `/about/`
- [ ] Services page - `/services/`
- [ ] 404 page - `/404.html` (trigger by visiting non-existent page)
- [ ] Search modal - Click search icon
- [ ] Mobile menu - Test on mobile viewport
- [ ] Dark mode - Test all pages in dark mode

### Component Testing Checklist
- [ ] Hero section
- [ ] Post grid/cards
- [ ] Post listing (blog page)
- [ ] Individual post content
- [ ] Author bio
- [ ] Share buttons
- [ ] Post navigation (prev/next)
- [ ] Tag cloud
- [ ] Pagination
- [ ] Navigation (desktop + mobile)
- [ ] Search modal
- [ ] Reading progress bar
- [ ] Footer

### Responsive Testing
- [ ] Desktop (1920px, 1440px, 1024px)
- [ ] Tablet (768px)
- [ ] Mobile (375px, 414px)

### Dark Mode Testing
- [ ] All pages in dark mode
- [ ] All interactive elements in dark mode
- [ ] Proper contrast ratios

---

## Template File Locations

```
templates/
├── base.html              # Base layout (nav, footer, search)
├── index.html             # Homepage
├── section.html           # Blog listing
├── page.html              # Individual posts/pages
├── 404.html               # 404 error page
├── tags/
│   ├── list.html          # All tags page
│   └── single.html        # Single tag page
└── categories/
    ├── list.html          # All categories page
    └── single.html        # Single category page
```

---

## Lessons Learned from This Migration

1. **Always test ALL page types**, not just the homepage
2. **Template hierarchy matters**: `section.html` ≠ `page.html` ≠ `index.html`
3. **Taxonomy pages are easy to miss** (tags/categories)
4. **Check both list and single views** for taxonomies
5. **Static pages use different templates** (About/Services use page.html)
6. **Test dark mode on all pages**, not just one
7. **Mobile menu has unique styles** separate from desktop
8. **Search modal is global** but easy to forget

---

## Quick Verification Commands

```bash
# Check all templates exist
find templates -name "*.html" -type f

# List all CSS classes used in templates
grep -rh 'class="[^"]*"' templates/ | grep -o 'class="[^"]*"' | sort -u

# Check for missing styles in CSS
grep -o 'class="[^"]*"' templates/**/*.html | cut -d'"' -f2 | tr ' ' '\n' | sort -u > used-classes.txt
grep -o '\.[a-zA-Z0-9_-]*' styles/input.css | cut -d'.' -f2 | sort -u > defined-classes.txt
comm -23 used-classes.txt defined-classes.txt  # Classes used but not defined
```

---

**Last Updated:** November 13, 2025
**Migration:** Tailwind CSS v4
**Status:** Complete - All Pages Verified ✅
