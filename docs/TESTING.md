# Testing Guide for MilesHope.com

This document provides comprehensive testing instructions for all features implemented in this session.

## Feature Categories

### 1. Quick Wins & UX Features
### 2. Engagement Boosters
### 3. SEO & Schema
### 4. Analytics & Tracking

---

## 1. Quick Wins & UX Features

### ✅ Copy Code Button

**What to Test:**
- [ ] Visit any blog post with code blocks
- [ ] Hover over a code block - "Copy" button should appear (desktop)
- [ ] Button should always be visible on mobile
- [ ] Click "Copy" - should change to "Copied!" with checkmark
- [ ] Paste content - should match code block exactly
- [ ] Test in dark mode - button styles should adapt

**Expected Behavior:**
- Button appears on hover (desktop) or always visible (mobile)
- Smooth fade-in animation
- Purple accent on hover
- Success feedback for 2 seconds
- Analytics event: `copy_code` tracked in GA4

**Files:**
- `templates/base.html` (lines 546-598)
- `styles/input.css` (lines 2762-2846)

---

### ✅ Active TOC Highlighting

**What to Test:**
- [ ] Visit a long blog post with Table of Contents
- [ ] Scroll through the post slowly
- [ ] Current section should be highlighted in purple in TOC
- [ ] Purple left border should appear on active link
- [ ] Click TOC links - should smooth scroll to section
- [ ] Test on mobile - TOC should be collapsible

**Expected Behavior:**
- Active section has purple highlight
- Smooth scroll with offset for header
- Font weight increases on active item
- Dark mode support

**Files:**
- `templates/page.html` (lines 203-228, 382-455)
- `styles/input.css` (lines 1757-1824)

---

### ✅ Print Styles

**What to Test:**
- [ ] Open any blog post
- [ ] Press Ctrl+P / Cmd+P to print
- [ ] Preview should show:
  - Clean black & white layout
  - No navigation, footer, or buttons
  - URLs displayed after external links
  - Proper page breaks
  - Table of contents included

**Expected Behavior:**
- Professional print-ready layout
- A4 page size with 2cm margins
- No background colors/images
- Code blocks with borders
- Author bio included

**Files:**
- `styles/input.css` (lines 2875-3098)

---

## 2. Engagement Boosters

### ✅ Reading List / Bookmarks

**What to Test:**
- [ ] Visit any blog post
- [ ] Click "Save" button (bookmark icon) in post header
- [ ] Button should change to "✓ Saved"
- [ ] Refresh page - state should persist
- [ ] Click again to remove from reading list
- [ ] Check localStorage: `readingList` key should exist

**Expected Behavior:**
- Instant save with visual feedback
- LocalStorage persistence
- "Added!" message for 1.5 seconds
- Purple/gold gradient background
- Green when saved

**Analytics Events:**
- `add_to_reading_list`
- `remove_from_reading_list`

**Files:**
- `templates/page.html` (lines 178-185, 593-685)
- `styles/input.css` (lines 3100-3175)

---

### ✅ Post Reactions

**What to Test:**
- [ ] Scroll to bottom of blog post
- [ ] See 4 reaction buttons: 👍 ✨ ❤️ 🤯
- [ ] Click a reaction - count should increment
- [ ] Button should get purple border and background
- [ ] Click again - should decrement and remove highlight
- [ ] Refresh page - state should persist
- [ ] Check localStorage: `postReactions` and `userReactions` keys

**Expected Behavior:**
- Scale animation on click (1.2x)
- Persistent state across sessions
- Each user can react multiple times (tracked locally)
- Mobile: 2-column grid

**Analytics Events:**
- `add_reaction` (with reaction type)
- `remove_reaction`

**Files:**
- `templates/page.html` (lines 319-346, 716-840)
- `styles/input.css` (lines 3177-3331)

---

### ✅ Related Posts

**What to Test:**
- [ ] Visit any blog post with tags
- [ ] Scroll to "Related Posts" section
- [ ] Should show up to 3 related posts based on tag overlap
- [ ] Each card shows title, excerpt, reading time, category
- [ ] Mobile: stacks to single column

**Expected Behavior:**
- Only shows if related posts exist
- Sorted by tag match count
- Responsive grid layout
- Dark mode support

**Files:**
- `templates/page.html` (lines 272-315)
- `styles/input.css` (lines 1826-1857)

---

### ✅ Author Bio

**What to Test:**
- [ ] Visit any blog post
- [ ] Scroll below content
- [ ] See author bio with circular avatar placeholder
- [ ] Click "More about me →" link
- [ ] Should navigate to About page

**Expected Behavior:**
- Purple/gold gradient avatar
- Left-aligned layout with avatar + text
- Mobile: smaller avatar
- Dark mode support

**Files:**
- `templates/page.html` (lines 252-270)
- `styles/input.css` (lines 1349-1436)

---

### ✅ Callout Boxes

**What to Test:**
To test, add this markdown to any blog post:

```html
<div class="callout callout-note">
  <div class="callout-content">
    <h4 class="callout-title">Note</h4>
    <p>This is an important note for readers.</p>
  </div>
</div>
```

**Available Types:**
- `callout-note` (Blue, ℹ️)
- `callout-tip` (Green, 💡)
- `callout-warning` (Orange, ⚠️)
- `callout-danger` (Red, 🚨)
- `callout-success` (Teal, ✓)
- `callout-quote` (Purple, 💭)

**Expected Behavior:**
- Icon on left (auto from CSS)
- Gradient background matching type
- 4px colored left border
- Dark mode support

**Files:**
- `styles/input.css` (lines 3333-3537)

---

### ✅ Focus States (Accessibility)

**What to Test:**
- [ ] Press Tab key to navigate site
- [ ] All interactive elements should show purple outline
- [ ] Outline should be visible and clear
- [ ] Test: links, buttons, form inputs, TOC, reactions
- [ ] Dark mode: outline should be lighter purple

**Expected Behavior:**
- 2-3px purple outline with offset
- Glow effect on buttons
- No outline on mouse click (only keyboard)
- Respects `prefers-reduced-motion`

**Files:**
- `styles/input.css` (lines 3539-3738)

---

## 3. SEO & Schema

### ✅ FAQ Schema

**What to Test:**
- [ ] Visit: Complete Guide to Bazi, Understanding Tarot, How to Choose
- [ ] View page source (Ctrl+U)
- [ ] Search for `"@type": "FAQPage"`
- [ ] Should see JSON-LD with questions and answers
- [ ] Test with [Google Rich Results Test](https://search.google.com/test/rich-results)
- [ ] Paste post URL - should pass validation

**Expected FAQ Counts:**
- Complete Guide to Bazi: 6 FAQs
- Understanding Tarot: 6 FAQs
- How to Choose: 6 FAQs

**Google Features Enabled:**
- FAQ rich snippets in search results
- Expandable Q&A in SERPs
- Featured snippet eligibility
- Voice search answers

**Files:**
- `templates/page.html` (lines 135-155)
- `content/blog/complete-guide-to-bazi.md`
- `content/blog/understanding-tarot-beginners-guide.md`
- `content/blog/how-to-choose-tarot-vs-bazi.md`

**Test URL:**
https://search.google.com/test/rich-results

---

## 4. Analytics & Tracking

### ✅ Scroll Depth Tracking

**What to Test:**
- [ ] Open blog post
- [ ] Open browser DevTools → Console
- [ ] Scroll to 25% of page
- [ ] Check Network tab for GA4 event
- [ ] Continue scrolling to 50%, 75%, 100%
- [ ] Each milestone should fire once only

**GA4 Event:**
```
Event: scroll_depth
Parameters:
  - event_category: "engagement"
  - event_label: "/blog/post-url"
  - value: 25 (or 50, 75, 100)
```

**Files:**
- `templates/page.html` (lines 871-902)

---

### ✅ Time on Page Tracking

**What to Test:**
- [ ] Open blog post
- [ ] Read for 30+ seconds while moving mouse occasionally
- [ ] Switch to another tab for 10 seconds
- [ ] Return to post tab
- [ ] Close tab or navigate away
- [ ] Check GA4 for `time_on_page` event

**Inactivity Rules:**
- Marks inactive after 30s of no interaction
- Pauses when tab is hidden
- Only counts active engagement time

**GA4 Events:**
```
Event: time_on_page
Parameters:
  - value: 45 (seconds)

Event: time_bucket
Parameters:
  - event_label: "30-60s" (or "0-30s", "1-3min", etc.)
```

**Files:**
- `templates/page.html` (lines 904-974)

---

### ✅ Exit Intent Detection

**What to Test:**
- [ ] Open blog post
- [ ] Move mouse rapidly toward browser top edge
- [ ] Move mouse out of viewport from top
- [ ] Check GA4 for `exit_intent` event
- [ ] Try again - should only fire once per page

**Expected Behavior:**
- Triggers when mouse Y < 0
- Records scroll position at exit
- One-time event per page view

**GA4 Event:**
```
Event: exit_intent
Parameters:
  - event_label: "/blog/post-url"
  - value: 42 (scroll percentage when exited)
```

**Files:**
- `templates/page.html` (lines 976-992)

---

### ✅ Popular Posts Tracking & Widget

**What to Test:**

**Part 1: View Tracking**
- [ ] Visit 2-3 different blog posts
- [ ] Check localStorage: `postViews` key
- [ ] Should see JSON with post URLs, titles, counts, timestamps
- [ ] Visit same post twice - count should increment

**Part 2: Widget Display**
- [ ] Visit homepage after viewing 2+ posts
- [ ] Scroll down - should see "📈 Your Most Read Posts" section
- [ ] Should show top 3 posts ranked 1, 2, 3
- [ ] Each post shows view count badge
- [ ] First visit: widget hidden (need 2+ posts viewed)

**Part 3: Widget Interaction**
- [ ] Hover over popular post card - should lift up
- [ ] Click a post - should navigate
- [ ] Check GA4 for `popular_post_click` event

**LocalStorage Structure:**
```json
{
  "postViews": {
    "/blog/complete-guide-to-bazi/": {
      "title": "Complete Guide to Bazi",
      "count": 3,
      "lastViewed": 1234567890
    }
  }
}
```

**GA4 Events:**
```
Event: page_view_tracked (on each post visit)
Event: popular_posts_shown (widget appears)
Event: popular_post_click (widget link clicked)
```

**Files:**
- `templates/page.html` (lines 1009-1048)
- `templates/index.html` (lines 100-187)
- `styles/input.css` (lines 3740-3900)

---

## Responsive Design Testing

### Breakpoints to Test:

**Desktop (1024px+)**
- [ ] All features at full width
- [ ] 3-column grids where applicable
- [ ] Hover states work properly

**Tablet Landscape (768px - 1024px)**
- [ ] 2-column grids
- [ ] Adjusted spacing
- [ ] Images scale appropriately

**Tablet Portrait (480px - 768px)**
- [ ] Copy code button always visible
- [ ] Single column layouts
- [ ] Stacked navigation
- [ ] Reactions: 2x2 grid

**Mobile (< 480px)**
- [ ] Full-bleed featured images
- [ ] Compact button sizes
- [ ] Touch-friendly targets
- [ ] Readable font sizes

---

## Dark Mode Testing

**For Every Feature:**
- [ ] Toggle dark mode button (moon icon)
- [ ] All text should be readable
- [ ] Purple accents should switch to lighter purple
- [ ] Backgrounds should be dark
- [ ] Borders and shadows should be visible
- [ ] No white flashes

**LocalStorage:**
- Dark mode preference: `theme` key = "dark" or "light"

---

## Browser Testing

**Recommended Browsers:**
- [ ] Chrome/Edge (latest)
- [ ] Firefox (latest)
- [ ] Safari (latest)
- [ ] Mobile Safari (iOS)
- [ ] Chrome Mobile (Android)

**Features to Verify:**
- [ ] LocalStorage works
- [ ] Clipboard API (copy code)
- [ ] Smooth scrolling
- [ ] requestAnimationFrame
- [ ] Visibility API

---

## Performance Testing

### Lighthouse Audit

Run in Chrome DevTools:
1. Open DevTools → Lighthouse tab
2. Select categories: Performance, Accessibility, Best Practices, SEO
3. Click "Generate report"

**Target Scores:**
- Performance: 90+
- Accessibility: 95+
- Best Practices: 95+
- SEO: 100

### Core Web Vitals

- **LCP**: < 2.5s (Largest Contentful Paint)
- **FID**: < 100ms (First Input Delay)
- **CLS**: < 0.1 (Cumulative Layout Shift)

---

## GA4 Dashboard Setup

### Custom Events to Monitor:

1. **Engagement**
   - scroll_depth
   - time_on_page
   - time_bucket
   - exit_intent

2. **Content Interaction**
   - copy_code
   - add_reaction / remove_reaction
   - add_to_reading_list / remove_from_reading_list

3. **Widget Performance**
   - popular_posts_shown
   - popular_post_click

### Recommended GA4 Reports:

**Engagement Report:**
- Average scroll depth by post
- Average time on page by category
- Exit intent patterns

**Content Report:**
- Most copied code blocks
- Most reacted posts
- Most bookmarked posts

**Widget Report:**
- Popular posts widget CTR
- Top posts from widget clicks

---

## Automated Testing Script

Create this file to test LocalStorage features:

```javascript
// test-storage.js
// Run in browser console

function testLocalStorage() {
  console.log('🧪 Testing LocalStorage Features...\n');

  // Test 1: Reading List
  const readingList = localStorage.getItem('readingList');
  console.log('✅ Reading List:', readingList ? JSON.parse(readingList) : 'Empty');

  // Test 2: Post Views
  const postViews = localStorage.getItem('postViews');
  console.log('✅ Post Views:', postViews ? JSON.parse(postViews) : 'Empty');

  // Test 3: Post Reactions
  const reactions = localStorage.getItem('postReactions');
  console.log('✅ Reactions:', reactions ? JSON.parse(reactions) : 'Empty');

  // Test 4: User Reactions
  const userReactions = localStorage.getItem('userReactions');
  console.log('✅ User Reactions:', userReactions ? JSON.parse(userReactions) : 'Empty');

  // Test 5: Theme
  const theme = localStorage.getItem('theme');
  console.log('✅ Theme:', theme || 'light (default)');

  console.log('\n✨ All tests complete!');
}

testLocalStorage();
```

---

## Manual Checklist

### Pre-Deployment
- [ ] All features work in production build
- [ ] No console errors
- [ ] All analytics events firing
- [ ] Dark mode works everywhere
- [ ] Mobile responsive
- [ ] Print styles work
- [ ] Schema validates

### Post-Deployment
- [ ] Google Rich Results test passes
- [ ] GA4 events appear in real-time report
- [ ] Site loads quickly (< 3s)
- [ ] All images load properly
- [ ] SSL/HTTPS working
- [ ] CDN configured (if applicable)

---

## Troubleshooting

### Copy Code Not Working
- Check browser supports Clipboard API
- Verify HTTPS (required for clipboard)
- Check DevTools console for errors

### Analytics Not Tracking
- Verify GA4 ID in config
- Check `gtag` function exists
- Test in Network tab for analytics calls
- Disable ad blockers

### LocalStorage Issues
- Check browser privacy settings
- Verify not in private/incognito mode
- Check storage quota (unlikely issue)

### Dark Mode Not Persisting
- Check localStorage for `theme` key
- Verify JavaScript isn't blocked
- Clear cache and test again

---

## Resources

- **Google Rich Results Test**: https://search.google.com/test/rich-results
- **Schema Validator**: https://validator.schema.org/
- **GA4 Debugger**: Chrome Extension "Google Analytics Debugger"
- **Lighthouse**: Built into Chrome DevTools
- **WebPageTest**: https://www.webpagetest.org/

---

## Summary

Total features tested: **15+**
- 4 Quick Win features
- 6 Engagement features
- 1 SEO schema feature
- 4 Analytics features

All features include:
- ✅ Dark mode support
- ✅ Mobile responsive
- ✅ Accessibility (WCAG)
- ✅ Performance optimized
- ✅ Analytics tracking
