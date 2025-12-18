# Colors

The MilesHope.com color system reflects the spiritual and welcoming nature of the brand through a carefully curated palette of purples and golds, complemented by neutral grays for content and structure.

## Color Philosophy

Our color choices convey:
- **Purple** - Wisdom, spirituality, mysticism, transformation
- **Gold** - Enlightenment, prosperity, warmth, divine knowledge
- **Grays** - Balance, clarity, grounding, readability

## Primary Colors

### Purple (Primary Brand Color)

Purple is our primary brand color, representing spirituality and wisdom.

| Color | Hex | CSS Variable | Usage |
|-------|-----|--------------|-------|
| Purple | `#805ad5` | `--color-purple` | Primary actions, links, headings in dark mode |
| Purple Light | `#b794f4` | `--color-purple-light` | Hover states, dark mode primary actions |
| Purple Dark | `#6b46c1` | `--color-purple-dark` | Gradients, depth, active states |
| Purple Darker | `#553c9a` | `--color-purple-darker` | Deep shadows, gradients |

**When to use**:
- Primary CTAs and important actions
- Links and interactive elements
- Hero gradients
- Brand elements (logo, featured content)

**Code example**:
```css
/* Using CSS variable */
.button-primary {
    background-color: var(--color-purple);
}

/* Light mode - purple links */
a {
    color: #805ad5; /* var(--color-purple) */
}

/* Dark mode - lighter purple for better contrast */
html.dark a {
    color: #b794f4; /* var(--color-purple-light) */
}
```

### Gold (Accent Color)

Gold provides warmth and highlights important elements without competing with purple.

| Color | Hex | CSS Variable | Usage |
|-------|-----|--------------|-------|
| Gold | `#d69e2e` | `--color-gold` | Accents, highlights, gradients |
| Gold Light | `#fbd38d` | `--color-gold-light` | Hover effects, dark mode accents, badges |

**When to use**:
- Reading time badges
- Tags and labels
- Gradient endpoints
- Accent elements that need warmth
- Secondary CTAs in dark mode

**Code example**:
```css
/* Reading time badge with gold gradient */
.reading-time {
    background: linear-gradient(135deg,
        rgba(213, 158, 46, 0.2),
        rgba(251, 211, 141, 0.2)
    );
    color: var(--color-gold);
}

/* Dark mode CTA button */
html.dark .btn-primary {
    background-color: var(--color-gold-light);
    color: var(--color-gray-800);
}
```

## Neutral Colors

### Gray Scale

A balanced gray scale for text, backgrounds, and structural elements.

| Color | Hex | CSS Variable | Usage |
|-------|-----|--------------|-------|
| Gray 100 | `#f7fafc` | `--color-gray-100` | Light mode background, cards |
| Gray 200 | `#e2e8f0` | `--color-gray-200` | Borders, dividers in light mode |
| Gray 400 | `#cbd5e0` | `--color-gray-400` | Muted text, placeholders |
| Gray 600 | `#4a5568` | `--color-gray-600` | Secondary text |
| Gray 700 | `#2d3748` | `--color-gray-700` | Primary text in light mode |
| Gray 800 | `#1a202c` | `--color-gray-800` | Dark mode background, darkest text |

**When to use**:
- **100-200**: Backgrounds, subtle borders (light mode)
- **400**: Muted text, disabled states, subtle elements
- **600**: Secondary information, metadata (reading time, dates)
- **700**: Body text, headings (light mode)
- **800**: Dark mode backgrounds, highest contrast text

**Code example**:
```css
/* Light mode body text */
body {
    color: var(--color-gray-700); /* #2d3748 */
    background-color: #ffffff;
}

/* Dark mode body text and background */
html.dark body {
    color: var(--color-gray-100); /* #f7fafc */
    background-color: var(--color-gray-800); /* #1a202c */
}
```

## Semantic Tokens

For easier maintainability, we define semantic tokens:

```css
--color-primary: var(--color-purple);
--color-accent: var(--color-gold);
```

**When to use semantic tokens**:
- Prefer `var(--color-primary)` over `var(--color-purple)` in component styles
- Makes color scheme changes easier (update one variable vs. many)
- Clearer intent in code

## Dark Mode Color Mapping

Dark mode uses inverted colors for backgrounds while maintaining or enhancing contrast ratios.

| Element | Light Mode | Dark Mode | Rationale |
|---------|-----------|-----------|-----------|
| Background | `#ffffff` | `#1a202c` (gray-800) | Deep, comfortable dark background |
| Text | `#2d3748` (gray-700) | `#f7fafc` (gray-100) | High contrast for readability |
| Primary CTA | `#ffffff` bg, `#805ad5` text | `#fbd38d` bg (gold-light), `#1a202c` text | Warm, inviting in dark mode |
| Secondary CTA | Transparent, `#ffffff` border | Transparent, `#fbd38d` border | Consistent with primary |
| Links | `#805ad5` (purple) | `#b794f4` (purple-light) | Lighter purple for better contrast |
| Borders | `#e2e8f0` (gray-200) | `#4a5568` (gray-600) | Subtle but visible |
| Cards | `#ffffff` | `#2d3748` (gray-700) | Elevated from background |

**Implementation**:
```css
/* Light mode (default) */
.post-card {
    background-color: #ffffff;
    border: 1px solid var(--color-gray-200);
    color: var(--color-gray-700);
}

/* Dark mode */
html.dark .post-card {
    background-color: var(--color-gray-700);
    border-color: var(--color-gray-600);
    color: var(--color-gray-100);
}
```

## Gradients

Our brand uses gradients to add depth and visual interest.

### Purple Gradient (Hero)

```css
background: linear-gradient(135deg, #805ad5 0%, #6b46c1 100%);
```

**Usage**: Hero sections, major headings, feature callouts

### Purple-to-Gold Gradient (Progress Bar)

```css
background: linear-gradient(90deg, #805ad5, #d69e2e);
```

**Usage**: Reading progress bar, loading indicators

### Gold Gradient (Text Highlights)

```css
background: linear-gradient(135deg, #d69e2e 0%, #fbd38d 100%);
-webkit-background-clip: text;
-webkit-text-fill-color: transparent;
background-clip: text;
```

**Usage**: Special headings, emphasized quotes

## Accessibility & Contrast

All color combinations meet **WCAG 2.1 Level AAA** standards for contrast.

### Contrast Ratios

| Combination | Ratio | Standard | Usage |
|-------------|-------|----------|-------|
| Gray 700 on White | 12.63:1 | AAA ✅ | Body text (light mode) |
| Gray 100 on Gray 800 | 15.8:1 | AAA ✅ | Body text (dark mode) |
| Purple on White | 4.54:1 | AA ✅ | Links, headings (min. 18px) |
| Purple Light on Gray 800 | 7.19:1 | AAA ✅ | Links (dark mode) |
| Gold on White | 3.03:1 | Large text only | Accents (min. 24px bold) |

**Testing tools**:
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- Chrome DevTools Lighthouse
- Browser extensions: axe DevTools, WAVE

### Best Practices

1. **Never use Gold on White for small text** - Contrast ratio is too low (3.03:1)
2. **Use Purple Light in dark mode** - Better contrast than standard purple
3. **Maintain 4.5:1 minimum** for body text (we exceed this with 12.63:1+)
4. **Test with color blindness simulators** - Ensure content is distinguishable

## Usage Guidelines

### Do's ✅

- Use purple for primary actions and brand elements
- Use gold for accents, warmth, and secondary highlights
- Maintain proper contrast ratios (WCAG AA minimum, AAA preferred)
- Use CSS variables for maintainability
- Provide dark mode variants for all colored elements
- Use gradients sparingly for emphasis

### Don'ts ❌

- Don't use gold for small body text (contrast issue)
- Don't mix purple and gold at equal prominence (confusing hierarchy)
- Don't use hardcoded hex values (use CSS variables)
- Don't skip dark mode testing
- Don't use colors alone to convey information (add icons, labels)
- Don't over-saturate (too much color fatigues users)

## Code Reference

**CSS Variables location**: `styles/theme.css`

```css
@theme {
    /* Primary palette */
    --color-purple: #805ad5;
    --color-purple-light: #b794f4;
    --color-purple-dark: #6b46c1;
    --color-purple-darker: #553c9a;
    --color-gold: #d69e2e;
    --color-gold-light: #fbd38d;

    /* Neutrals */
    --color-gray-600: #4a5568;
    --color-gray-700: #2d3748;
    --color-gray-800: #1a202c;
    --color-gray-100: #f7fafc;
    --color-gray-200: #e2e8f0;
    --color-gray-400: #cbd5e0;

    /* Semantic tokens */
    --color-primary: var(--color-purple);
    --color-accent: var(--color-gold);
}
```

**Dark mode implementation**: `templates/base.html` (JavaScript toggle) + CSS `.dark` class selectors

## Related Documentation

- [Typography](typography.md) - Text color usage with type styles
- [Accessibility](accessibility.md) - WCAG compliance and contrast testing
- [Dark Mode Pattern](../patterns/dark-mode.md) - Complete dark mode implementation guide
- [Buttons](../components/buttons.md) - Color usage in button variants

---

**Last updated**: January 2025
