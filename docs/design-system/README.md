# MilesHope.com Design System

Welcome to the MilesHope.com design system documentation. This comprehensive guide documents the visual language, components, and patterns that create the spiritual and modern aesthetic of MilesHope.com.

## Philosophy & Principles

The MilesHope.com design system embodies three core principles:

### 1. **Spiritual Elegance**
Our design reflects the spiritual nature of our content through:
- Warm, inviting purple and gold color palette
- Elegant serif typography (Lora) for headings that evoke wisdom and tradition
- Thoughtful use of gradients and subtle animations
- Spacious layouts that encourage mindful reading

### 2. **Modern Accessibility**
We prioritize user experience across all contexts:
- WCAG 2.1 AAA compliance for color contrast
- Fully responsive design (mobile-first approach)
- Comprehensive dark mode support
- Keyboard navigation and screen reader optimization
- Fast, performant interactions

### 3. **Systematic Consistency**
Built on Tailwind CSS v4 with:
- Design tokens via CSS custom properties
- Modular, maintainable architecture
- Fluid typography scales
- Reusable component patterns
- Documented conventions

## Tech Stack

- **Framework**: [Tailwind CSS v4](https://tailwindcss.com/) (standalone CLI)
- **Build Tool**: Tailwind CSS CLI + Zola static site generator
- **CSS Architecture**: Modular CSS with custom properties
- **Dark Mode**: Class-based (`.dark` on `<html>`)
- **Typography**: Google Fonts (Lora + Inter)

## Documentation Structure

### 🎨 Foundation
Core design tokens and foundational styles:

- **[Colors](foundation/colors.md)** - Purple/gold palette, variants, dark mode mapping
- **[Typography](foundation/typography.md)** - Font families, fluid scales, usage guidelines
- **[Spacing](foundation/spacing.md)** - Spacing patterns, grid system, layout conventions
- **[Accessibility](foundation/accessibility.md)** - WCAG compliance, focus states, ARIA patterns

### 🧩 Components
Reusable UI components with code examples:

- **[Buttons](components/buttons.md)** - Primary and secondary button variants
- **[Cards](components/cards.md)** - Post cards with hover effects
- **[Navigation](components/navigation.md)** - Header, mobile menu, breadcrumbs
- **[Interactive](components/interactive.md)** - Search, theme toggle, TOC, reactions
- **[Content](components/content.md)** - Hero sections, post grids, author bio

### 📐 Patterns
Implementation patterns and best practices:

- **[Dark Mode](patterns/dark-mode.md)** - Implementation strategy, color mapping, transitions
- **[Responsive](patterns/responsive.md)** - Breakpoint strategy, mobile patterns, testing
- **[Animations](patterns/animations.md)** - Transitions, hover effects, performance

## Quick Start

### For Developers

1. **Explore Foundation** - Start with [Colors](foundation/colors.md) and [Typography](foundation/typography.md) to understand core design tokens
2. **Review Components** - Check [Components](components/) for reusable patterns with code examples
3. **Understand Patterns** - Read [Patterns](patterns/) for system-wide implementation guidelines
4. **Build New Features** - Use documented components and maintain consistency

### For Designers

1. **Review Color Palette** - [Colors](foundation/colors.md) documents the purple/gold spiritual aesthetic
2. **Typography System** - [Typography](foundation/typography.md) explains our font choices and scales
3. **Component Library** - Browse [Components](components/) to see all available UI patterns
4. **Accessibility** - [Accessibility](foundation/accessibility.md) ensures inclusive design

## CSS Architecture

Our CSS is organized into 7 modular files (804 lines total):

```
styles/
├── input.css                    # Entry point (imports all modules)
├── theme.css                    # CSS custom properties + Tailwind @theme
├── base.css                     # Foundation styles (typography, links)
├── utilities.css                # Custom utilities (gradients, progress bar)
└── components/
    ├── layout.css              # Header, navigation, footer, container
    ├── content.css             # Buttons, cards, hero, post grids
    └── interactive.css         # Search, theme toggle, mobile menu
```

**Compiled output**: `/static/css/tailwind.css` (minified for production)

## Design Tokens

All core values are defined as CSS custom properties in `styles/theme.css`:

```css
/* Colors */
--color-purple: #805ad5
--color-gold: #d69e2e

/* Typography */
--font-family-heading: 'Lora', Georgia, serif
--font-family-body: 'Inter', -apple-system, sans-serif
--font-size-h1: clamp(2rem, 1.5rem + 2.5vw, 3rem)

/* Spacing */
--container-max-width: 1200px
--section-spacing: 4rem
```

See [Foundation](foundation/) documentation for complete token reference.

## Key Features

### Implemented Components

- ✅ **Responsive Navigation** - Hamburger menu for mobile, full nav for desktop
- ✅ **Search Modal** - Client-side elasticlunr.js search with results
- ✅ **Dark Mode Toggle** - Persistent localStorage-based theme switching
- ✅ **Reading Progress Bar** - Purple-to-gold gradient scroll indicator
- ✅ **Table of Contents** - Expandable TOC with active section highlighting
- ✅ **Post Reactions** - Emoji reaction system with persistence
- ✅ **Reading List** - Bookmark posts to reading list
- ✅ **Related Posts** - Tag-based post recommendations
- ✅ **Author Bio** - Consistent author information display
- ✅ **Breadcrumbs** - Semantic navigation with ARIA support

### Advanced Features

- **Fluid Typography** - Responsive font sizing using `clamp()`
- **CSS Custom Properties** - Design tokens for maintainability
- **Dark Mode** - Comprehensive dark mode with smooth transitions
- **Accessibility** - WCAG 2.1 compliant with keyboard navigation
- **Performance** - requestAnimationFrame, passive listeners, optimized animations
- **SEO** - Semantic HTML, proper heading hierarchy, structured data

## Browser Support

- **Modern Browsers**: Chrome, Firefox, Safari, Edge (latest 2 versions)
- **Mobile**: iOS Safari 12+, Chrome Android
- **Features**: CSS custom properties, CSS Grid, Flexbox, `clamp()`

Progressive enhancement ensures core content is accessible on all browsers.

## Contributing

When adding new components or patterns:

1. **Follow Existing Patterns** - Review similar components in this documentation
2. **Use Design Tokens** - Prefer CSS variables over hardcoded values
3. **Support Dark Mode** - Include `.dark` variants for all colors
4. **Test Accessibility** - Verify keyboard navigation and screen reader support
5. **Document** - Add code examples and usage guidelines

## File Reference

**CSS Source Files**:
- `styles/theme.css` - Design tokens and Tailwind theme
- `styles/base.css` - Base element styles
- `styles/components/*.css` - Component-specific styles
- `styles/utilities.css` - Custom utility classes

**Templates**:
- `templates/base.html` - Base layout with header, footer, scripts
- `templates/index.html` - Homepage with hero and post grid
- `templates/page.html` - Blog post template with all interactive components
- `templates/section.html` - Blog listing with pagination

## Resources

- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [MDN Web Docs - CSS](https://developer.mozilla.org/en-US/docs/Web/CSS)
- [Google Fonts - Lora](https://fonts.google.com/specimen/Lora)
- [Google Fonts - Inter](https://fonts.google.com/specimen/Inter)

---

**Version**: 1.0.0 (January 2025)
**Maintained by**: MilesHope.com
**License**: Copyright 2025 MilesHope.com. All rights reserved.
