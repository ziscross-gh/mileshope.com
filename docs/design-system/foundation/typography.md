# Typography

The MilesHope.com typography system combines the elegant serif Lora for headings with the clean sans-serif Inter for body text, creating a balance between traditional wisdom and modern readability.

## Font Philosophy

### Headings: Lora (Serif)

**Lora** is an elegant, contemporary serif typeface with moderate contrast and distinctive letterforms.

**Why Lora**:
- Evokes wisdom and tradition (appropriate for spiritual content)
- Excellent readability even at large sizes
- Distinctive character without being overly decorative
- Works well with both light and bold weights
- Professional yet approachable

**Characteristics**:
- **Style**: Contemporary serif with calligraphic influence
- **Weights used**: 400 (Regular), 500 (Medium), 600 (Semi-Bold), 700 (Bold)
- **Supports**: Regular and Italic styles
- **Fallback**: Georgia, "Times New Roman", serif

**When to use**:
- All heading levels (H1-H6)
- Hero titles and featured content
- Pull quotes and blockquotes
- Emphasized text that needs elegance

### Body: Inter (Sans-Serif)

**Inter** is a carefully crafted sans-serif designed specifically for computer screens.

**Why Inter**:
- Optimized for readability at small sizes
- Clean, modern aesthetic
- Excellent x-height for legibility
- Wide character set with special glyphs
- Complements Lora without competing

**Characteristics**:
- **Style**: Humanist sans-serif optimized for UI
- **Weights used**: 400 (Regular), 500 (Medium), 600 (Semi-Bold), 700 (Bold)
- **Features**: Tabular numbers, contextual alternates
- **Fallback**: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif

**When to use**:
- All body text and paragraphs
- Navigation menus
- Form labels and inputs
- Buttons and UI elements
- Metadata (dates, reading time, tags)

## Font Families

### CSS Variables

```css
--font-serif: 'Lora', Georgia, "Times New Roman", serif;
--font-sans: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
```

### Loading Fonts

Fonts are loaded from Google Fonts with `display=swap` for performance:

```html
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Lora:ital,wght@0,400;0,500;0,600;0,700;1,400;1,500;1,600;1,700&display=swap" rel="stylesheet">
```

**Fallback stack**:
- System fonts load instantly while web fonts download
- Minimizes layout shift (font metrics are similar)
- Graceful degradation on slow connections

## Type Scale

Our type scale uses **fluid typography** with CSS `clamp()` for responsive sizing without media queries.

### Fluid Typography Formula

```css
font-size: clamp([min-size], [preferred-size], [max-size]);
```

- **min-size**: Smallest size (mobile)
- **preferred-size**: Calculated with viewport units
- **max-size**: Largest size (desktop)

### Heading Sizes

| Element | Mobile (min) | Desktop (max) | CSS Variable | Line Height |
|---------|--------------|---------------|--------------|-------------|
| **H1** | 2rem (32px) | 3rem (48px) | `--font-size-h1` | 1.2 |
| **H2** | 1.5rem (24px) | 2.25rem (36px) | `--font-size-h2` | 1.3 |
| **H3** | 1.25rem (20px) | 1.75rem (28px) | `--font-size-h3` | 1.3 |
| **H4** | 1.125rem (18px) | 1.5rem (24px) | `--font-size-h4` | 1.4 |
| **H5** | 1rem (16px) | 1.25rem (20px) | `--font-size-h5` | 1.4 |
| **H6** | 0.875rem (14px) | 1rem (16px) | `--font-size-h6` | 1.4 |

**Implementation**:
```css
h1 {
    font-size: var(--font-size-h1);
    font-size: clamp(2rem, 1.5rem + 2.5vw, 3rem);
    font-family: var(--font-serif);
    font-weight: 600;
    line-height: 1.2;
}
```

### Body Sizes

| Element | Mobile (min) | Desktop (max) | CSS Variable | Usage |
|---------|--------------|---------------|--------------|-------|
| **Base** | 1rem (16px) | 1.125rem (18px) | `--font-size-base` | Body text, paragraphs |
| **Small** | 0.875rem (14px) | 0.9375rem (15px) | `--font-size-sm` | Captions, footnotes |
| **Extra Small** | 0.75rem (12px) | 0.8125rem (13px) | `--font-size-xs` | Tiny labels, metadata |

**Implementation**:
```css
body {
    font-size: var(--font-size-base);
    font-size: clamp(1rem, 0.95rem + 0.25vw, 1.125rem);
    font-family: var(--font-sans);
    line-height: 1.6;
}
```

## Text Styles

### Headings

All headings use Lora (serif), bold weight, and reduced letter-spacing for elegance.

```css
h1, h2, h3, h4, h5, h6 {
    font-family: var(--font-serif);
    font-weight: 600;
    letter-spacing: 0.01em;
    margin-bottom: 1rem;
    color: var(--color-gray-700);
}

html.dark h1,
html.dark h2,
html.dark h3,
html.dark h4,
html.dark h5,
html.dark h6 {
    color: var(--color-gray-100);
}
```

**Usage guidelines**:
- **H1**: Page titles, hero headings (one per page)
- **H2**: Section headings, major content divisions
- **H3**: Subsection headings, card titles
- **H4**: Minor headings within subsections
- **H5**: Rare, for deep content hierarchy
- **H6**: Rarely used, smallest heading

### Body Text

Body text uses Inter (sans-serif) with generous line-height for comfortable reading.

```css
p {
    font-family: var(--font-sans);
    font-size: var(--font-size-base);
    line-height: 1.6;
    margin-bottom: 1rem;
    color: var(--color-gray-700);
}

html.dark p {
    color: var(--color-gray-100);
}
```

**Optimal line length**: 50-75 characters (achieved with `max-width: 65ch` on content)

### Links

Links use purple color with subtle underline on hover.

```css
a {
    color: var(--color-purple);
    text-decoration: none;
    transition: color 0.3s ease;
}

a:hover {
    text-decoration: underline;
}

html.dark a {
    color: var(--color-purple-light);
}
```

### Emphasis

```css
/* Bold */
strong, b {
    font-weight: 600;
    color: var(--color-gray-800);
}

html.dark strong,
html.dark b {
    color: var(--color-gray-100);
}

/* Italic */
em, i {
    font-style: italic;
    font-family: var(--font-serif); /* Lora italic is beautiful */
}

/* Code */
code {
    font-family: 'Monaco', 'Consolas', monospace;
    font-size: 0.9em;
    padding: 0.2em 0.4em;
    background-color: rgba(128, 90, 213, 0.1);
    border-radius: 4px;
    color: var(--color-purple-dark);
}
```

### Blockquotes

Blockquotes use Lora italic for elegant, distinguished quotes.

```css
blockquote {
    font-family: var(--font-serif);
    font-style: italic;
    font-size: clamp(1.125rem, 1rem + 0.5vw, 1.25rem);
    line-height: 1.5;
    margin: 2rem 0;
    padding-left: 1.5rem;
    border-left: 4px solid var(--color-gold);
    color: var(--color-gray-600);
}

html.dark blockquote {
    border-left-color: var(--color-gold-light);
    color: var(--color-gray-400);
}
```

## Line Heights

Optimal line-height varies by font size and line length:

| Element | Line Height | Rationale |
|---------|-------------|-----------|
| **H1** | 1.2 | Large text needs less spacing |
| **H2-H3** | 1.3 | Balance readability with compactness |
| **H4-H6** | 1.4 | Smaller headings need more breathing room |
| **Body** | 1.6 | Optimal for sustained reading |
| **Small text** | 1.5 | Compact but readable |

## Letter Spacing

Subtle letter-spacing adjustments improve readability:

```css
/* Headings - slight tracking for elegance */
h1, h2, h3, h4, h5, h6 {
    letter-spacing: 0.01em;
}

/* Uppercase text - increase tracking for legibility */
.uppercase {
    letter-spacing: 0.05em;
}

/* Body text - default spacing (0) is optimal for Inter */
p {
    letter-spacing: 0;
}
```

## Responsive Typography

Typography scales fluidly across all screen sizes using `clamp()`:

### Mobile (< 768px)

- **H1**: 32px (2rem)
- **Body**: 16px (1rem)
- **Line length**: Full width with 1rem padding

### Tablet (768px - 1024px)

- **H1**: ~40px (interpolated)
- **Body**: ~17px (interpolated)
- **Line length**: `max-width: 65ch`

### Desktop (> 1024px)

- **H1**: 48px (3rem)
- **Body**: 18px (1.125rem)
- **Line length**: `max-width: 65ch`

**No media queries needed** - `clamp()` handles all breakpoints smoothly.

## Special Typography

### Hero Headings

Extra-large, attention-grabbing headings for hero sections.

```css
.hero-title {
    font-family: var(--font-serif);
    font-size: clamp(2.5rem, 2rem + 3vw, 4rem);
    font-weight: 700;
    line-height: 1.1;
    letter-spacing: -0.01em; /* Negative tracking for large sizes */
    color: #ffffff;
}
```

### Post Titles

Blog post titles with specific styling for consistency.

```css
.post-title {
    font-family: var(--font-serif);
    font-size: var(--font-size-h1);
    font-weight: 600;
    line-height: 1.2;
    margin-bottom: 1.5rem;
    color: var(--color-gray-800);
}

html.dark .post-title {
    color: var(--color-gray-100);
}
```

### Metadata Text

Small, muted text for dates, reading time, author info.

```css
.post-meta {
    font-family: var(--font-sans);
    font-size: var(--font-size-sm);
    color: var(--color-gray-600);
    display: flex;
    gap: 1rem;
}

html.dark .post-meta {
    color: var(--color-gray-400);
}
```

## Accessibility

### Font Size Minimums

- **Never** use font-size smaller than 14px (0.875rem) for body text
- **Minimum** for small text: 12px (0.75rem), only for labels/metadata
- **Respect** user browser settings (use `rem` units, not `px`)

### Color Contrast

- All text meets **WCAG 2.1 Level AAA** contrast (7:1 minimum for normal text)
- Gray 700 on white: **12.63:1** ✅
- Gray 100 on gray 800: **15.8:1** ✅

### Readability Best Practices

1. **Limit line length** - Max 65-75 characters for optimal reading
2. **Generous line-height** - 1.6 for body text, 1.2-1.4 for headings
3. **Clear hierarchy** - Distinct sizes between heading levels
4. **Sufficient contrast** - 4.5:1 minimum, 7:1 preferred
5. **Responsive sizes** - Text scales appropriately on all devices

## Usage Guidelines

### Do's ✅

- Use Lora for all headings and quoted text
- Use Inter for body copy, UI elements, and navigation
- Maintain clear hierarchy (don't skip heading levels)
- Use `rem` units for accessibility
- Keep line length between 50-75 characters
- Use appropriate line-heights (1.6 for body, 1.2-1.3 for headings)
- Test with browser zoom (150-200%)

### Don'ts ❌

- Don't use more than 2 font families (we use Lora + Inter)
- Don't manually adjust font-size with media queries (use `clamp()`)
- Don't use font-size below 14px for readable content
- Don't set line-height below 1.4 for body text
- Don't use all-caps extensively (reduces readability)
- Don't skip heading levels (H1 → H3)

## Code Reference

**Font variables**: `styles/theme.css`

```css
@theme {
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    --font-serif: 'Lora', Georgia, "Times New Roman", serif;
}

:root {
    --font-size-base: clamp(1rem, 0.95rem + 0.25vw, 1.125rem);
    --font-size-h1: clamp(2rem, 1.5rem + 2.5vw, 3rem);
    --font-size-h2: clamp(1.5rem, 1.25rem + 1.25vw, 2.25rem);
    --font-size-h3: clamp(1.25rem, 1.1rem + 0.75vw, 1.75rem);
    --font-size-h4: clamp(1.125rem, 1rem + 0.5vw, 1.5rem);
    --font-size-h5: clamp(1rem, 0.95rem + 0.25vw, 1.25rem);
    --font-size-h6: clamp(0.875rem, 0.85rem + 0.125vw, 1rem);
    --font-size-sm: clamp(0.875rem, 0.85rem + 0.125vw, 0.9375rem);
    --font-size-xs: clamp(0.75rem, 0.725rem + 0.125vw, 0.8125rem);
}
```

**Base styles**: `styles/base.css`

## Related Documentation

- [Colors](colors.md) - Text color usage and contrast ratios
- [Accessibility](accessibility.md) - Font size, contrast, and readability standards
- [Responsive Pattern](../patterns/responsive.md) - How typography adapts across devices

---

**Last updated**: January 2025
