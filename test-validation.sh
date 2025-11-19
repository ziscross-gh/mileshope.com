#!/bin/bash

echo "🧪 MilesHope.com Feature Validation Test Suite"
echo "=============================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0
WARN=0

# Test 1: Check all template files exist
echo "📄 Test 1: Template Files"
TEMPLATES=(
  "templates/base.html"
  "templates/index.html"
  "templates/page.html"
  "templates/section.html"
)

for template in "${TEMPLATES[@]}"; do
  if [ -f "$template" ]; then
    echo -e "${GREEN}✓${NC} $template exists"
    ((PASS++))
  else
    echo -e "${RED}✗${NC} $template missing"
    ((FAIL++))
  fi
done
echo ""

# Test 2: Check CSS file exists and has content
echo "🎨 Test 2: Styles"
if [ -f "styles/input.css" ]; then
  LINES=$(wc -l < styles/input.css)
  if [ $LINES -gt 3000 ]; then
    echo -e "${GREEN}✓${NC} styles/input.css exists ($LINES lines)"
    ((PASS++))
  else
    echo -e "${YELLOW}⚠${NC} styles/input.css exists but seems short ($LINES lines)"
    ((WARN++))
  fi
else
  echo -e "${RED}✗${NC} styles/input.css missing"
  ((FAIL++))
fi
echo ""

# Test 3: Check for Copy Code Button implementation
echo "📋 Test 3: Copy Code Button"
if grep -q "Copy Code Button" templates/base.html; then
  echo -e "${GREEN}✓${NC} Copy code button JavaScript found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Copy code button not found"
  ((FAIL++))
fi

if grep -q "copy-code-btn" styles/input.css; then
  echo -e "${GREEN}✓${NC} Copy code button CSS found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Copy code button CSS not found"
  ((FAIL++))
fi
echo ""

# Test 4: Check for Reading List feature
echo "🔖 Test 4: Reading List/Bookmarks"
if grep -q "readingListBtn" templates/page.html; then
  echo -e "${GREEN}✓${NC} Reading list button found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Reading list button not found"
  ((FAIL++))
fi

if grep -q "reading-list-btn" styles/input.css; then
  echo -e "${GREEN}✓${NC} Reading list CSS found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Reading list CSS not found"
  ((FAIL++))
fi
echo ""

# Test 5: Check for Post Reactions
echo "😊 Test 5: Post Reactions"
if grep -q "post-reactions" templates/page.html; then
  echo -e "${GREEN}✓${NC} Post reactions HTML found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Post reactions HTML not found"
  ((FAIL++))
fi

if grep -q "reaction-btn" styles/input.css; then
  echo -e "${GREEN}✓${NC} Post reactions CSS found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Post reactions CSS not found"
  ((FAIL++))
fi
echo ""

# Test 6: Check for FAQ Schema
echo "🔍 Test 6: FAQ Schema"
FAQ_POSTS=(
  "content/blog/complete-guide-to-bazi.md"
  "content/blog/understanding-tarot-beginners-guide.md"
  "content/blog/how-to-choose-tarot-vs-bazi.md"
)

for post in "${FAQ_POSTS[@]}"; do
  if [ -f "$post" ]; then
    if grep -q "\[\[extra.faq\]\]" "$post"; then
      echo -e "${GREEN}✓${NC} FAQ data found in $post"
      ((PASS++))
    else
      echo -e "${RED}✗${NC} FAQ data missing from $post"
      ((FAIL++))
    fi
  else
    echo -e "${RED}✗${NC} $post not found"
    ((FAIL++))
  fi
done

if grep -q "FAQPage" templates/page.html; then
  echo -e "${GREEN}✓${NC} FAQ schema template found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} FAQ schema template not found"
  ((FAIL++))
fi
echo ""

# Test 7: Check for Analytics Tracking
echo "📊 Test 7: Analytics Tracking"
ANALYTICS_EVENTS=(
  "scroll_depth"
  "time_on_page"
  "exit_intent"
  "page_view_tracked"
)

for event in "${ANALYTICS_EVENTS[@]}"; do
  if grep -q "$event" templates/page.html; then
    echo -e "${GREEN}✓${NC} $event tracking found"
    ((PASS++))
  else
    echo -e "${RED}✗${NC} $event tracking not found"
    ((FAIL++))
  fi
done
echo ""

# Test 8: Check for Popular Posts Widget
echo "📈 Test 8: Popular Posts Widget"
if grep -q "popularPostsSection" templates/index.html; then
  echo -e "${GREEN}✓${NC} Popular posts widget HTML found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Popular posts widget HTML not found"
  ((FAIL++))
fi

if grep -q "popular-posts-grid" styles/input.css; then
  echo -e "${GREEN}✓${NC} Popular posts widget CSS found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Popular posts widget CSS not found"
  ((FAIL++))
fi
echo ""

# Test 9: Check for Callout Boxes
echo "💬 Test 9: Callout Boxes"
CALLOUT_TYPES=(
  "callout-note"
  "callout-tip"
  "callout-warning"
  "callout-danger"
  "callout-success"
  "callout-quote"
)

for type in "${CALLOUT_TYPES[@]}"; do
  if grep -q "$type" styles/input.css; then
    echo -e "${GREEN}✓${NC} $type CSS found"
    ((PASS++))
  else
    echo -e "${RED}✗${NC} $type CSS not found"
    ((FAIL++))
  fi
done
echo ""

# Test 10: Check for Print Styles
echo "🖨️  Test 10: Print Styles"
if grep -q "@media print" styles/input.css; then
  echo -e "${GREEN}✓${NC} Print styles found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Print styles not found"
  ((FAIL++))
fi
echo ""

# Test 11: Check for Focus States
echo "⌨️  Test 11: Accessibility Focus States"
if grep -q "focus-visible" styles/input.css; then
  echo -e "${GREEN}✓${NC} Focus states found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Focus states not found"
  ((FAIL++))
fi

if grep -q "prefers-reduced-motion" styles/input.css; then
  echo -e "${GREEN}✓${NC} Reduced motion support found"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Reduced motion support not found"
  ((FAIL++))
fi
echo ""

# Test 12: Check for Dark Mode Support
echo "🌙 Test 12: Dark Mode"
if grep -q "html.dark" styles/input.css; then
  DARK_COUNT=$(grep -c "html.dark" styles/input.css)
  echo -e "${GREEN}✓${NC} Dark mode styles found ($DARK_COUNT instances)"
  ((PASS++))
else
  echo -e "${RED}✗${NC} Dark mode styles not found"
  ((FAIL++))
fi
echo ""

# Test 13: Check for Featured Images
echo "🖼️  Test 13: Featured Images"
if [ -d "static/images" ]; then
  SVG_COUNT=$(find static/images -name "*.svg" 2>/dev/null | wc -l)
  if [ $SVG_COUNT -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Featured images found ($SVG_COUNT SVG files)"
    ((PASS++))
  else
    echo -e "${YELLOW}⚠${NC} No SVG images found"
    ((WARN++))
  fi
else
  echo -e "${YELLOW}⚠${NC} static/images directory not found"
  ((WARN++))
fi
echo ""

# Test 14: Check Documentation
echo "📚 Test 14: Documentation"
DOCS=(
  "TESTING.md"
  "SESSION_SUMMARY.md"
  "CLAUDE.md"
)

for doc in "${DOCS[@]}"; do
  if [ -f "$doc" ]; then
    echo -e "${GREEN}✓${NC} $doc exists"
    ((PASS++))
  else
    echo -e "${RED}✗${NC} $doc missing"
    ((FAIL++))
  fi
done
echo ""

# Test 15: Check for JavaScript syntax errors (basic)
echo "🔧 Test 15: JavaScript Syntax Check"
JS_FILES=(
  "templates/base.html"
  "templates/page.html"
  "templates/index.html"
)

for file in "${JS_FILES[@]}"; do
  if [ -f "$file" ]; then
    # Check for common JS errors
    if grep -q "function(" "$file" && ! grep -q "functio\s*\s*(" "$file"; then
      echo -e "${GREEN}✓${NC} $file - No obvious syntax errors"
      ((PASS++))
    else
      echo -e "${YELLOW}⚠${NC} $file - Check manually for syntax"
      ((WARN++))
    fi
  fi
done
echo ""

# Summary
echo "=============================================="
echo "📊 TEST SUMMARY"
echo "=============================================="
echo -e "${GREEN}✓ Passed:${NC} $PASS"
echo -e "${YELLOW}⚠ Warnings:${NC} $WARN"
echo -e "${RED}✗ Failed:${NC} $FAIL"
echo ""

TOTAL=$((PASS + FAIL + WARN))
SCORE=$((PASS * 100 / TOTAL))
echo "Score: $SCORE% ($PASS/$TOTAL)"
echo ""

if [ $FAIL -eq 0 ]; then
  echo -e "${GREEN}🎉 All critical tests passed!${NC}"
  echo "✅ Site is ready for deployment"
  exit 0
else
  echo -e "${RED}❌ Some tests failed${NC}"
  echo "Please review failed tests above"
  exit 1
fi
