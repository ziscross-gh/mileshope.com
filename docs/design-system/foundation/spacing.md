# Spacing

The MilesHope.com spacing system creates visual rhythm and hierarchy through consistent use of spacing values. Our spacing scale is based on a 0.25rem (4px) base unit for precision and flexibility.

## Spacing Philosophy

Proper spacing creates:
- **Visual hierarchy** - Larger spacing separates major sections
- **Breathing room** - Content feels inviting and not cramped
- **Readability** - Adequate spacing improves comprehension
- **Balance** - Consistent patterns create harmony

## Spacing Scale

Our spacing system uses `rem` units for accessibility (respects user font-size preferences).

### Base Scale

| Token | Value | Pixels (at 16px base) | Usage |
|-------|-------|-----------------------|-------|
| `xs` | 0.25rem | 4px | Tight spacing, icon gaps |
| `sm` | 0.5rem | 8px | Small gaps, compact elements |
| `md` | 1rem | 16px | Default spacing, standard gaps |
| `lg` | 1.5rem | 24px | Medium sections, card padding |
| `xl` | 2rem | 32px | Large sections, hero spacing |
| `2xl` | 3rem | 48px | Major section dividers |
| `3xl` | 4rem | 64px | Hero sections, page sections |
| `4xl` | 5rem | 80px | Maximum spacing, special sections |

### Common Spacing Patterns

| Pattern | Value | Usage |
|---------|-------|-------|
| **Inline elements** | 0.5rem (8px) | Buttons, badges, tags |
| **Component padding** | 0.875rem (14px) | Button padding, small cards |
| **Card spacing** | 1rem (16px) | Standard card internal padding |
| **Section padding** | 4rem (64px) | Major page sections |
| **Container padding** | 1rem (16px) | Side padding for mobile |
| **Grid gaps** | 2rem (32px) | Post grid spacing |

## Container System

### Container Widths

```css
.container {
    max-width: 1200px; /* var(--container-max-w) */
    margin: 0 auto;
    padding: 0 1rem; /* var(--container-padding) */
}
```

**Rationale**:
- **1200px max-width**: Optimal for readability and modern displays
- **Centered**: `margin: 0 auto` centers content
- **Side padding**: 1rem (16px) provides breathing room on mobile

**Usage**:
- Wrap all page content in `.container`
- Ensures consistent max-width across pages
- Responsive padding automatically applied

### Content Width (Prose)

For optimal readability, long-form content has additional width constraints:

```css
.post-content {
    max-width: 65ch; /* ~65 characters per line */
}
```

**Why 65ch**:
- Optimal line length for comfortable reading
- 50-75 characters is the recommended range
- `ch` unit adapts to font-size changes

## Section Spacing

### Vertical Rhythm

Consistent vertical spacing creates visual rhythm:

```css
/* Major sections */
.hero {
    padding: 5rem 2rem; /* Top/bottom: 80px, Sides: 32px */
}

/* Standard sections */
.recent-posts {
    padding: 4rem 0; /* Top/bottom: 64px */
}

/* Compact sections */
.site-header {
    padding: 1rem 0; /* Top/bottom: 16px */
}
```

**Pattern**: Use multiples of 1rem (16px) for vertical spacing

### Section Padding Values

| Section Type | Padding | Usage |
|--------------|---------|-------|
| **Hero** | 5rem 2rem (80px 32px) | Large, attention-grabbing sections |
| **Major sections** | 4rem 0 (64px 0) | Blog listings, featured content |
| **Medium sections** | 3rem 0 (48px 0) | Secondary content areas |
| **Compact sections** | 2rem 0 (32px 0) | Footer, minor sections |
| **Navigation** | 1rem 0 (16px 0) | Header, compact UI |

## Component Spacing

### Buttons

```css
.btn {
    padding: 0.875rem 2rem; /* Vertical: 14px, Horizontal: 32px */
}
```

**Rationale**:
- Generous horizontal padding (2rem) for comfortable click targets
- Moderate vertical padding (0.875rem) for balance
- Minimum 44px × 44px for accessibility (WCAG)

### Cards

```css
.post-card {
    padding: 1.5rem; /* 24px all around */
    gap: 1rem; /* 16px between elements */
}
```

**Internal spacing**:
- **Card padding**: 1.5rem (24px) creates breathing room
- **Element gaps**: 1rem (16px) separates title, excerpt, metadata

### Grid Layouts

```css
.post-grid {
    display: grid;
    gap: 2rem; /* 32px between cards */
    grid-template-columns: repeat(3, 1fr);
}

@media (max-width: 1024px) {
    .post-grid {
        gap: 1.5rem; /* 24px on tablets */
    }
}

@media (max-width: 768px) {
    .post-grid {
        gap: 1.5rem; /* 24px on mobile */
    }
}
```

**Grid spacing strategy**:
- **Desktop (3 columns)**: 2rem gap for clear separation
- **Tablet (2 columns)**: 1.5rem gap (tighter but still comfortable)
- **Mobile (1 column)**: 1.5rem vertical gap between stacked cards

### Navigation

```css
.nav-menu {
    gap: 3rem; /* 48px between nav items (desktop) */
}

.navbar .container {
    gap: 3rem; /* 48px between logo and menu */
}
```

**Spacing rationale**:
- Large gaps (3rem) create distinct clickable areas
- Prevents accidental clicks on adjacent items
- Feels open and uncluttered

## Typography Spacing

### Heading Margins

```css
h1, h2, h3, h4, h5, h6 {
    margin-bottom: 1rem; /* 16px below headings */
}
```

**Why 1rem**:
- Creates clear separation from following content
- Consistent across all heading levels
- Not too tight, not too loose

### Paragraph Margins

```css
p {
    margin-bottom: 1rem; /* 16px between paragraphs */
}
```

**Optimal for**:
- Comfortable reading flow
- Clear paragraph separation
- Maintains vertical rhythm

### List Spacing

```css
ul, ol {
    margin-bottom: 1rem;
    padding-left: 1.5rem; /* 24px indent */
}

li {
    margin-bottom: 0.5rem; /* 8px between list items */
}
```

## Responsive Spacing

Spacing adapts to screen size for optimal use of space:

### Mobile (<768px)

```css
.hero {
    padding: 3rem 1.5rem; /* Reduced from 5rem 2rem */
}

.container {
    padding: 0 1rem; /* 16px side padding */
}
```

**Strategy**:
- Reduce large spacing values by 30-40%
- Maintain minimum padding of 1rem (16px)
- Keep text readable with adequate margins

### Tablet (768px - 1024px)

```css
.recent-posts {
    padding: 3rem 0; /* Slightly reduced from 4rem */
}
```

**Strategy**:
- Intermediate spacing between mobile and desktop
- Reduce by ~25% from desktop values
- Maintain hierarchy

### Desktop (>1024px)

Full spacing values as designed:
- Hero: 5rem vertical
- Sections: 4rem vertical
- Grids: 2rem gaps

## Special Spacing Patterns

### Inline Elements

Small spacing for inline elements like tags:

```css
.tag {
    padding: 0.375rem 0.875rem; /* 6px × 14px */
    margin-right: 0.5rem; /* 8px gap between tags */
}
```

### Flexbox Gaps

Prefer `gap` property over margins for flex layouts:

```css
/* Modern approach */
.post-meta {
    display: flex;
    gap: 1rem; /* 16px between elements */
}

/* Old approach (avoid) */
.post-meta > * {
    margin-right: 1rem; /* Less flexible */
}
```

**Advantages of `gap`**:
- No margin on last element
- Cleaner code
- Works with wrapping content

### Grid Template Gaps

```css
.post-grid {
    display: grid;
    gap: 2rem; /* 32px between all grid items */
    /* Applies to both row and column gaps */
}

/* Or specify separately */
.custom-grid {
    row-gap: 2rem; /* 32px vertical */
    column-gap: 1.5rem; /* 24px horizontal */
}
```

## Spacing Utilities

While Tailwind provides spacing utilities, we primarily use custom classes for consistency.

### Common Tailwind Spacing Classes

- `p-4` = 1rem (16px) padding all sides
- `px-6` = 1.5rem (24px) horizontal padding
- `py-8` = 2rem (32px) vertical padding
- `gap-4` = 1rem (16px) gap
- `mb-4` = 1rem (16px) margin-bottom

**When to use**:
- Quick prototyping
- One-off spacing adjustments
- Consistent with design tokens

## Accessibility Considerations

### Touch Target Sizes

All interactive elements meet **WCAG 2.1 Level AAA** touch target size (44px × 44px minimum):

```css
.btn {
    padding: 0.875rem 2rem; /* Results in ~48px height */
}

.nav-menu a {
    padding: 0.5rem 1rem; /* Results in ~44px height with font-size */
}
```

### Keyboard Navigation

Adequate spacing ensures clear focus indicators:

```css
a:focus {
    outline: 2px solid var(--color-purple);
    outline-offset: 2px; /* 2px gap between element and outline */
}
```

## Usage Guidelines

### Do's ✅

- Use `rem` units for spacing (respects user preferences)
- Stick to the spacing scale (0.5rem, 1rem, 1.5rem, 2rem, etc.)
- Use `gap` property for flex/grid layouts
- Maintain consistent vertical rhythm (multiples of 1rem)
- Test spacing on all breakpoints
- Use `max-width` to limit content width for readability

### Don'ts ❌

- Don't use arbitrary spacing values (use scale)
- Don't use `px` for spacing (use `rem`)
- Don't create cramped layouts (minimum 1rem padding)
- Don't ignore mobile spacing adjustments
- Don't skip spacing between major sections
- Don't exceed 75 characters line length for body text

## Code Reference

**Container variables**: `styles/theme.css`

```css
@theme {
    --container-max-w: 1200px;
    --container-padding: 1rem;
}
```

**Layout spacing**: `styles/components/layout.css`
**Content spacing**: `styles/components/content.css`
**Component spacing**: `styles/components/interactive.css`

## Spacing Examples

### Card Component

```css
.post-card {
    padding: 1.5rem; /* Internal padding */
    margin-bottom: 2rem; /* Space below card */
    gap: 1rem; /* Gap between internal elements */
}
```

### Section Layout

```css
.recent-posts {
    padding: 4rem 0; /* Vertical section spacing */
}

.recent-posts .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem; /* Horizontal container padding */
}
```

### Navigation

```css
.navbar .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 3rem; /* Large gap for distinct areas */
}

.nav-menu {
    display: flex;
    gap: 3rem; /* Space between nav items */
}
```

## Related Documentation

- [Typography](typography.md) - Text spacing, line-height, margins
- [Responsive Pattern](../patterns/responsive.md) - How spacing adapts across breakpoints
- [Accessibility](accessibility.md) - Touch target sizes and spacing requirements

---

**Last updated**: January 2025
