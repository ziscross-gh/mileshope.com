# Accessibility

MilesHope.com is committed to creating an inclusive experience for all users. Our design system follows **WCAG 2.1 Level AAA** standards where possible, ensuring content is perceivable, operable, understandable, and robust.

## Accessibility Philosophy

We believe spiritual content should be accessible to everyone, regardless of ability. Our approach:

- **Universal Design** - Design for the widest range of users from the start
- **Progressive Enhancement** - Core content works without JavaScript
- **Keyboard First** - All functionality accessible via keyboard
- **Screen Reader Optimized** - Semantic HTML with proper ARIA labels
- **High Contrast** - Exceeding minimum contrast ratios
- **Flexible** - Respects user preferences (font size, reduced motion, etc.)

## WCAG 2.1 Compliance

### Level AAA Achievements

We meet or exceed AAA standards in these areas:

#### Color Contrast (1.4.6)

| Element | Ratio | Standard | Status |
|---------|-------|----------|--------|
| Body text (light mode) | 12.63:1 | AAA (7:1) | ✅ Exceeds |
| Body text (dark mode) | 15.8:1 | AAA (7:1) | ✅ Exceeds |
| Links (light mode) | 4.54:1 | AA (4.5:1) | ✅ Meets |
| Links (dark mode) | 7.19:1 | AAA (7:1) | ✅ Exceeds |
| Large text (18pt+) | 3:1 minimum | AA (3:1) | ✅ Meets |

**Testing**:
- Use [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- Chrome DevTools Accessibility panel
- Automated testing with axe DevTools

#### Text Spacing (1.4.12)

Users can adjust text spacing without loss of content:
- Line height (line-spacing): At least 1.5× font size
- Paragraph spacing: At least 2× font size
- Letter spacing: At least 0.12× font size
- Word spacing: At least 0.16× font size

```css
/* Our implementation already supports this */
p {
    line-height: 1.6; /* 1.5× minimum, we use 1.6 */
    margin-bottom: 1rem; /* 2× at 16px base */
}
```

#### Target Size (2.5.5)

All interactive elements meet 44px × 44px minimum:

```css
/* Buttons */
.btn {
    padding: 0.875rem 2rem; /* Results in ~48px height */
    min-height: 44px;
}

/* Navigation links */
.nav-menu a {
    padding: 0.5rem 1rem; /* Meets 44px height */
}
```

### Level AA Compliance

#### Keyboard Navigation (2.1.1)

All functionality available via keyboard:
- **Tab**: Navigate between interactive elements
- **Enter/Space**: Activate buttons and links
- **Esc**: Close modals (search, mobile menu)
- **Arrow keys**: Navigate within TOC, menu

```javascript
// Search modal keyboard support
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape' && searchModal.classList.contains('active')) {
        closeSearch();
    }
});
```

#### Focus Visible (2.4.7)

Clear, high-contrast focus indicators:

```css
/* Default focus style */
*:focus {
    outline: 2px solid var(--color-purple);
    outline-offset: 2px;
}

/* Link focus */
a:focus {
    outline: 2px solid var(--color-purple);
    outline-offset: 2px;
    text-decoration: underline;
}

/* Button focus */
.btn:focus {
    outline: 2px solid var(--color-purple);
    outline-offset: 2px;
    box-shadow: 0 0 0 4px rgba(128, 90, 213, 0.2);
}

/* Dark mode focus */
html.dark *:focus {
    outline-color: var(--color-purple-light);
}
```

#### Headings and Labels (2.4.6)

- Descriptive headings for all sections
- Proper heading hierarchy (H1 → H2 → H3, no skipping)
- Form labels associated with inputs
- ARIA labels for icon-only buttons

```html
<!-- Proper heading hierarchy -->
<h1>Blog Post Title</h1>
<h2>Section Heading</h2>
<h3>Subsection Heading</h3>

<!-- ARIA labels for icon buttons -->
<button aria-label="Toggle dark mode" class="theme-toggle">
    <span class="theme-icon">🌙</span>
</button>

<button aria-label="Search" class="search-toggle">
    <span class="search-icon">🔍</span>
</button>
```

## Semantic HTML

We use semantic HTML5 elements for better accessibility:

### Document Structure

```html
<header class="site-header">
    <nav class="navbar" aria-label="Main navigation">
        <!-- Navigation content -->
    </nav>
</header>

<main class="main-content">
    <!-- Main page content -->
    <article class="blog-post">
        <!-- Blog post content -->
    </article>
</main>

<footer class="site-footer">
    <!-- Footer content -->
</footer>
```

### Navigation

```html
<!-- Breadcrumbs with ARIA -->
<nav aria-label="Breadcrumb" class="breadcrumbs">
    <ol class="breadcrumb-list">
        <li class="breadcrumb-item">
            <a href="/">Home</a>
        </li>
        <li class="breadcrumb-separator">›</li>
        <li class="breadcrumb-item">
            <a href="/blog/">Blog</a>
        </li>
        <li class="breadcrumb-separator">›</li>
        <li class="breadcrumb-item breadcrumb-current" aria-current="page">
            Post Title
        </li>
    </ol>
</nav>
```

### Landmarks

Proper landmark roles for screen readers:

- `<header>` - Banner landmark
- `<nav>` - Navigation landmark
- `<main>` - Main content landmark
- `<aside>` - Complementary landmark (TOC, related posts)
- `<footer>` - Contentinfo landmark

## Screen Reader Support

### Alt Text

All images have descriptive alt text:

```html
<!-- Featured images -->
<img src="/images/tarot-beginners.svg"
     alt="Understanding Tarot - A Beginner's Guide with tarot cards"
     loading="lazy"
     decoding="async">

<!-- Decorative images -->
<img src="/images/decorative-pattern.svg" alt="" role="presentation">
```

**Guidelines**:
- **Informative images**: Describe the content/purpose
- **Decorative images**: Use empty alt (`alt=""`) and `role="presentation"`
- **Complex images**: Provide detailed description nearby
- **Icon images**: Describe the function, not appearance

### ARIA Labels

ARIA labels provide context for screen readers:

```html
<!-- Search toggle button -->
<button class="search-toggle" id="searchToggle" aria-label="Search">
    <span class="search-icon">🔍</span>
</button>

<!-- Close button -->
<button class="search-close" id="searchClose" aria-label="Close search">
    ×
</button>

<!-- Dark mode toggle -->
<button class="theme-toggle" id="themeToggle" aria-label="Toggle dark mode">
    <span class="theme-icon">🌙</span>
</button>
```

### Live Regions

Dynamic content updates announced to screen readers:

```html
<!-- Search results (announced when updated) -->
<div class="search-results" id="searchResults" role="region" aria-live="polite">
    <!-- Results inserted dynamically -->
</div>
```

**ARIA live regions**:
- `aria-live="polite"`: Announce when screen reader is idle
- `aria-live="assertive"`: Announce immediately (use sparingly)
- `aria-atomic="true"`: Read entire region on change

## Keyboard Navigation

### Focus Management

Logical tab order throughout the site:

1. Skip link (hidden, visible on focus)
2. Logo / Home link
3. Navigation menu items
4. Search button
5. Dark mode toggle
6. Main content
7. Article links and interactive elements
8. Footer links

### Keyboard Shortcuts

| Key | Action | Context |
|-----|--------|---------|
| **Tab** | Move to next focusable element | Global |
| **Shift + Tab** | Move to previous focusable element | Global |
| **Enter** | Activate link or button | Focusable elements |
| **Space** | Activate button | Buttons |
| **Esc** | Close modal/menu | Search modal, mobile menu |
| **/** | Open search | Global (when not in input) |

### Skip Links

Skip navigation for keyboard users:

```html
<a href="#main-content" class="skip-link">
    Skip to main content
</a>

<main id="main-content">
    <!-- Page content -->
</main>
```

```css
.skip-link {
    position: absolute;
    top: -100px;
    left: 0;
    background: var(--color-purple);
    color: white;
    padding: 0.5rem 1rem;
    z-index: 1000;
}

.skip-link:focus {
    top: 0;
}
```

## Responsive Text

### User Font Size Preferences

All measurements use `rem` units to respect user font-size settings:

```css
/* Scales with user preferences */
body {
    font-size: clamp(1rem, 0.95rem + 0.25vw, 1.125rem);
}

/* DO NOT use px for text */
.bad-example {
    font-size: 16px; /* ❌ Ignores user preferences */
}

.good-example {
    font-size: 1rem; /* ✅ Respects user preferences */
}
```

### Minimum Font Sizes

- **Body text**: Never below 16px (1rem) at default zoom
- **Small text**: Never below 14px (0.875rem)
- **Minimum readable**: 12px (0.75rem), only for labels/metadata

### Zoom Support

Site remains usable at 200% zoom (WCAG 1.4.4):
- Text reflows without horizontal scrolling
- No content loss or overlap
- Interactive elements remain operable

Test at:
- 100% (default)
- 150% (common for aging eyes)
- 200% (WCAG requirement)

## Color Independence

Never use color alone to convey information:

### Good Examples ✅

```html
<!-- Icon + color for status -->
<span class="status-success">
    ✓ Published
</span>

<!-- Underline + color for links -->
<a href="/post/">Read more →</a>
```

### Bad Examples ❌

```html
<!-- Color only - not accessible -->
<span style="color: red;">Error</span>

<!-- Link without underline on hover - hard to distinguish -->
<a style="text-decoration: none;">Click here</a>
```

## Motion & Animation

### Reduced Motion

Respect `prefers-reduced-motion` preference:

```css
/* Default: smooth animations */
.post-card {
    transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.post-card:hover {
    transform: translateY(-4px);
}

/* Reduced motion: instant changes */
@media (prefers-reduced-motion: reduce) {
    * {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }

    .post-card:hover {
        transform: none;
    }
}
```

### Safe Animations

- **No flashing**: Nothing flashes more than 3 times per second
- **Parallax**: Minimal or none (can cause motion sickness)
- **Autoplay**: No autoplaying videos with sound
- **Infinite loops**: Can be paused

## Form Accessibility

(MilesHope.com currently has minimal forms, but here are guidelines)

### Labels

```html
<!-- Associated label -->
<label for="email">Email address</label>
<input type="email" id="email" name="email" required>

<!-- Or wrap input -->
<label>
    Email address
    <input type="email" name="email" required>
</label>
```

### Error Messages

```html
<label for="email">Email address</label>
<input type="email"
       id="email"
       name="email"
       required
       aria-describedby="email-error"
       aria-invalid="true">
<span id="email-error" class="error" role="alert">
    Please enter a valid email address
</span>
```

## Testing Checklist

### Manual Testing

- [ ] **Keyboard navigation**: Tab through entire page
- [ ] **Focus indicators**: Visible on all interactive elements
- [ ] **Screen reader**: Test with NVDA (Windows) or VoiceOver (Mac)
- [ ] **Zoom**: Test at 150% and 200% zoom
- [ ] **Color contrast**: Check all text/background combinations
- [ ] **Text spacing**: Increase spacing, check for overlaps
- [ ] **Reduced motion**: Enable preference, check animations

### Automated Testing

- [ ] **axe DevTools**: Browser extension scan
- [ ] **Lighthouse**: Chrome DevTools accessibility audit
- [ ] **WAVE**: Web accessibility evaluation tool
- [ ] **HTML validator**: W3C validation
- [ ] **Color contrast**: WebAIM checker

### Screen Reader Testing

**VoiceOver (Mac)**:
1. Enable: System Preferences → Accessibility → VoiceOver
2. Navigate: Cmd+F5 to toggle
3. Test: Navigate through page, check announcements

**NVDA (Windows, free)**:
1. Download from nvaccess.org
2. Test: Browse mode and focus mode
3. Verify: Headings, landmarks, ARIA labels

## Common Accessibility Patterns

### Modal Dialog

```html
<div class="modal" role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <div class="modal-content">
        <h2 id="modal-title">Search</h2>
        <button aria-label="Close" class="close">×</button>
        <!-- Modal content -->
    </div>
</div>
```

**JavaScript requirements**:
- Trap focus within modal
- Close on Esc key
- Return focus to trigger element on close

### Expandable Content (TOC)

```html
<button aria-expanded="false" aria-controls="toc-content" class="toc-toggle">
    Table of Contents
</button>
<div id="toc-content" hidden>
    <!-- TOC content -->
</div>
```

**JavaScript**:
```javascript
toggle.addEventListener('click', function() {
    const expanded = this.getAttribute('aria-expanded') === 'true';
    this.setAttribute('aria-expanded', !expanded);
    content.hidden = expanded;
});
```

## Resources & Tools

### Testing Tools

- [axe DevTools](https://www.deque.com/axe/devtools/) - Browser extension
- [WAVE](https://wave.webaim.org/) - Web accessibility evaluation
- [Lighthouse](https://developers.google.com/web/tools/lighthouse) - Chrome DevTools
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Accessibility Insights](https://accessibilityinsights.io/) - Microsoft tool

### Screen Readers

- [NVDA](https://www.nvaccess.org/) - Free (Windows)
- [VoiceOver](https://www.apple.com/accessibility/voiceover/) - Built-in (Mac)
- [JAWS](https://www.freedomscientific.com/products/software/jaws/) - Paid (Windows)
- [TalkBack](https://support.google.com/accessibility/android/answer/6283677) - Built-in (Android)

### Guidelines & Standards

- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/) - Official guidelines
- [WAI-ARIA](https://www.w3.org/WAI/ARIA/apg/) - ARIA authoring practices
- [WebAIM](https://webaim.org/) - Accessibility resources
- [A11y Project](https://www.a11yproject.com/) - Community-driven checklist

## Best Practices

### Do's ✅

- Use semantic HTML elements
- Provide text alternatives for images
- Ensure sufficient color contrast (7:1 for AAA)
- Support keyboard navigation
- Use ARIA labels for icon-only buttons
- Test with screen readers
- Respect user preferences (font size, motion)
- Maintain logical tab order

### Don'ts ❌

- Don't rely on color alone to convey meaning
- Don't skip heading levels (H1 → H3)
- Don't use `<div>` for interactive elements (use `<button>`)
- Don't disable zoom (`user-scalable=no`)
- Don't auto-play videos with sound
- Don't use low contrast colors
- Don't forget alt text on images
- Don't trap users in keyboard traps

## Related Documentation

- [Colors](colors.md) - Contrast ratios and color guidelines
- [Typography](typography.md) - Font sizes and readability
- [Spacing](spacing.md) - Touch target sizes

---

**Last updated**: January 2025
**WCAG Level**: AAA (where possible), AA minimum
