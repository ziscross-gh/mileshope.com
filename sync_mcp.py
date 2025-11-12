#!/usr/bin/env python3
"""
Notion to Zola Sync Tool (MCP Version)
Syncs published blog posts from Notion database to Zola markdown files.
Uses MCP Notion integration - no API key needed!
"""

import json
import re
from pathlib import Path
from typing import Dict, List, Optional
import sys

# This script expects to be run via Claude Code with Notion MCP
# It reads from stdin expecting JSON data from the Notion MCP fetch

def slugify(text: str) -> str:
    """Convert title to URL-friendly slug."""
    text = text.lower()
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '-', text)
    return text.strip('-')

def parse_tags_categories(text: str) -> List[str]:
    """Parse space or comma-separated tags/categories."""
    if not text:
        return []

    # Split by comma or space
    items = re.split(r'[,\s]+', text.strip())
    # Filter empty strings
    return [item.strip() for item in items if item.strip()]

def create_frontmatter(properties: Dict) -> str:
    """Generate Zola TOML frontmatter from Notion properties."""
    title = properties.get("Title", "Untitled")
    date = properties.get("date:Publish Date:start", "2025-01-12")
    description = properties.get("SEO Description", "")

    # Parse tags and categories from text fields
    tags_text = properties.get("Tags", "")
    category_text = properties.get("Category", "")

    tags = parse_tags_categories(tags_text)
    categories = parse_tags_categories(category_text)

    # Build frontmatter
    frontmatter = "+++\n"
    frontmatter += f'title = "{title}"\n'
    frontmatter += f'date = {date}\n'

    if description:
        # Escape quotes in description
        description = description.replace('"', '\\"')
        frontmatter += f'description = "{description}"\n'

    if tags or categories:
        frontmatter += "\n[taxonomies]\n"
        if categories:
            categories_str = ", ".join([f'"{cat}"' for cat in categories])
            frontmatter += f"categories = [{categories_str}]\n"
        if tags:
            tags_str = ", ".join([f'"{tag}"' for tag in tags])
            frontmatter += f"tags = [{tags_str}]\n"

    frontmatter += "+++\n\n"
    return frontmatter

def format_content(content: str) -> str:
    """Format content text into proper paragraphs."""
    if not content:
        return ""

    # Split into sentences and create paragraphs
    # This is basic formatting - adjust as needed
    paragraphs = content.split('\n')
    formatted = []

    for para in paragraphs:
        para = para.strip()
        if para:
            formatted.append(para)

    return '\n\n'.join(formatted) + '\n'

def sync_post(page_data: Dict) -> bool:
    """Sync a single post from Notion to Zola."""
    properties = page_data.get("properties", {})

    title = properties.get("Title", "Untitled")
    status = properties.get("Status", "").strip()

    # Only sync published posts
    if status != "Published":
        print(f"  ⊘ Skipping '{title}' (Status: {status})")
        return False

    # Use provided slug or generate from title
    slug = properties.get("Slug", "").strip()
    if not slug:
        slug = slugify(title)

    filename = f"{slug}.md"
    filepath = Path("content/blog") / filename

    print(f"  → Syncing: {title}")

    # Generate frontmatter
    frontmatter = create_frontmatter(properties)

    # Get content from property
    content = properties.get("Content", "")
    formatted_content = format_content(content)

    # Write file
    filepath.parent.mkdir(parents=True, exist_ok=True)
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(frontmatter)
        f.write(formatted_content)

    print(f"    ✓ Written to {filepath}")
    return True

def main():
    """Main sync function."""
    print("🔄 Starting Notion to Zola sync (MCP mode)...\n")

    # In MCP mode, we expect page data to be provided
    # For now, let's create a test with the data we know exists
    print("📝 Note: This is a template script.")
    print("    Run via Claude Code with MCP integration for full functionality.\n")

    print("✅ Sync script is ready!")
    print("\nTo use with MCP:")
    print("  1. Ask Claude Code to fetch your Notion database")
    print("  2. Claude will process each page and call this script")
    print("  3. Or use the standalone sync.py with API keys")

if __name__ == "__main__":
    main()
