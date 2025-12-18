# Design System

**Note**: This file provides a quick overview. For comprehensive professional documentation, see **[design-system/README.md](design-system/README.md)**.

## Overview

MilesHope.com uses a custom design system built on **Tailwind CSS v4** with a focus on:
- **Spiritual elegance** through purple/gold color palette and Lora serif typography
- **Modern accessibility** with WCAG 2.1 AAA compliance
- **Systematic consistency** via design tokens and reusable components

## 📚 Full Documentation

Our design system is documented across multiple focused guides:

### 🎨 Foundation

Core design tokens and foundational styles:
- **[Colors](design-system/foundation/colors.md)** - Purple/gold palette, dark mode mapping, contrast ratios
- **[Typography](design-system/foundation/typography.md)** - Lora + Inter fonts, fluid scales, usage guidelines
- **[Spacing](design-system/foundation/spacing.md)** - Spacing scale, grid system, layout conventions
- **[Accessibility](design-system/foundation/accessibility.md)** - WCAG compliance, keyboard navigation, screen readers

### 🧩 Components

Reusable UI components (documentation coming soon):
- Buttons - Primary and secondary variants
- Cards - Post cards with hover effects
- Navigation - Header, mobile menu, breadcrumbs
- Interactive - Search, theme toggle, TOC, reactions
- Content - Hero sections, post grids, author bio

### 📐 Patterns

Implementation patterns (documentation coming soon):
- Dark Mode - Class-based system with localStorage
- Responsive - Breakpoint strategy and mobile patterns
- Animations - Transitions and performance

## Quick Reference

### Color Palette

| Color | Hex | Usage |
|-------|-----|-------|
| Purple | `#805ad5` | Primary brand, links, CTAs |
| Purple Light | `#b794f4` | Dark mode primary |
| Gold | `#d69e2e` | Accents, badges, highlights |
| Gold Light | `#fbd38d` | Dark mode CTAs |
| Gray 800 | `#1a202c` | Dark mode background |
| Gray 700 | `#2d3748` | Body text (light mode) |
| Gray 100 | `#f7fafc` | Light backgrounds, dark mode text |

See [Colors documentation](design-system/foundation/colors.md) for complete palette and usage guidelines.

### Typography

| Element | Font | Size Range | Usage |
|---------|------|------------|-------|
| **Headings** | Lora (serif) | 32px - 48px (H1) | All heading levels |
| **Body** | Inter (sans-serif) | 16px - 18px | Paragraphs, UI text |

Fluid typography scales automatically using `clamp()`. See [Typography documentation](design-system/foundation/typography.md).

### Components

| Component | Class | Description |
|-----------|-------|-------------|
| **Button Primary** | `.btn-primary` | Purple (light) / Gold (dark) CTAs |
| **Button Secondary** | `.btn-secondary` | Outline style buttons |
| **Post Card** | `.post-card` | Blog post cards with hover effects |
| **Container** | `.container` | Max-width 1200px wrapper |
| **Hero** | `.hero` | Purple gradient hero sections |

## File Structure

```
styles/
├── input.css                    # Entry point (imports all modules)
├── theme.css                    # Design tokens + Tailwind @theme
├── base.css                     # Foundation styles
├── utilities.css                # Custom utilities
└── components/
    ├── layout.css              # Header, navigation, footer
    ├── content.css             # Buttons, cards, hero sections
    └── interactive.css         # Search, dark mode toggle
```

**Compiled output**: `/static/css/tailwind.css`

## Getting Started

1. **Read the [Design System README](design-system/README.md)** for philosophy and overview
2. **Review [Foundation](design-system/foundation/)** to understand core design tokens
3. **Reference components** for reusable patterns (coming soon)
4. **Follow patterns** for system-wide implementation guidelines (coming soon)

## Contributing

When building new features:

1. Use CSS variables from `theme.css` instead of hardcoded values
2. Follow established spacing scale (0.5rem, 1rem, 1.5rem, 2rem, etc.)
3. Support dark mode with `.dark` class selectors
4. Test accessibility (keyboard navigation, screen readers, contrast)
5. Document new components in design system guides

## Resources

- **Full Documentation**: [design-system/README.md](design-system/README.md)
- **Tailwind CSS v4**: https://tailwindcss.com/docs
- **WCAG 2.1 Guidelines**: https://www.w3.org/WAI/WCAG21/quickref/
- **Google Fonts**: [Lora](https://fonts.google.com/specimen/Lora) | [Inter](https://fonts.google.com/specimen/Inter)

---

**Version**: 1.0.0
**Last Updated**: January 2025
**Maintained by**: MilesHope.com
