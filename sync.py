#!/usr/bin/env python3
"""
Notion to Zola Sync Tool
Syncs published blog posts from Notion database to Zola markdown files.
Optimized for MilesHope.com database structure.
"""

import os
import re
import requests
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional
import sys

# Configuration from environment variables
NOTION_API_KEY = os.getenv("NOTION_API_KEY")
NOTION_DATABASE_ID = os.getenv("NOTION_DATABASE_ID", "232ae70a30d480dd9eebe04b46260adf")

if not NOTION_API_KEY:
    print("Error: NOTION_API_KEY must be set in environment")
    print("Set it in your shell or create a .env file")
    sys.exit(1)

NOTION_VERSION = "2022-06-28"
HEADERS = {
    "Authorization": f"Bearer {NOTION_API_KEY}",
    "Notion-Version": NOTION_VERSION,
    "Content-Type": "application/json"
}

def query_published_posts() -> List[Dict]:
    """Query Notion database for published posts."""
    url = f"https://api.notion.com/v1/databases/{NOTION_DATABASE_ID}/query"

    # Filter for published posts (Status is text field, not status type)
    payload = {
        "filter": {
            "property": "Status",
            "rich_text": {
                "equals": "Published"
            }
        },
        "sorts": [
            {
                "property": "Publish Date",
                "direction": "descending"
            }
        ]
    }

    try:
        response = requests.post(url, json=payload, headers=HEADERS)
        response.raise_for_status()
        return response.json().get("results", [])
    except requests.exceptions.HTTPError as e:
        # If the filter fails, try without filter (Status might be different type)
        print("⚠️  Filter failed, fetching all posts...")
        payload = {
            "sorts": [
                {
                    "property": "Publish Date",
                    "direction": "descending"
                }
            ]
        }
        response = requests.post(url, json=payload, headers=HEADERS)
        response.raise_for_status()
        return response.json().get("results", [])

def extract_text_property(properties: Dict, prop_name: str) -> str:
    """Extract text from a text or rich_text property."""
    prop = properties.get(prop_name, {})
    prop_type = prop.get("type", "")

    if prop_type == "rich_text":
        rich_text = prop.get("rich_text", [])
        if rich_text:
            return "".join([rt.get("plain_text", "") for rt in rich_text])
    elif prop_type == "text":
        text = prop.get("text", [])
        if text:
            return "".join([t.get("plain_text", "") for t in text])

    return ""

def extract_title_property(properties: Dict, prop_name: str) -> str:
    """Extract text from a title property."""
    prop = properties.get(prop_name, {})
    title_list = prop.get("title", [])
    if title_list:
        return "".join([t.get("plain_text", "") for t in title_list])
    return ""

def extract_date_property(properties: Dict, prop_name: str) -> Optional[str]:
    """Extract date from a date property."""
    prop = properties.get(prop_name, {})
    date_obj = prop.get("date")
    if date_obj:
        return date_obj.get("start")
    return None

def extract_select_property(properties: Dict, prop_name: str) -> Optional[str]:
    """Extract value from a select property."""
    prop = properties.get(prop_name, {})
    select_obj = prop.get("select")
    if select_obj:
        return select_obj.get("name")
    return None

def parse_tags_categories(text: str) -> List[str]:
    """Parse space or comma-separated tags/categories."""
    if not text:
        return []

    # Split by comma, space, or hyphen
    items = re.split(r'[,\s]+', text.strip())
    # Filter empty strings
    return [item.strip() for item in items if item.strip()]

def slugify(text: str) -> str:
    """Convert title to URL-friendly slug."""
    text = text.lower()
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '-', text)
    return text.strip('-')

def create_frontmatter(properties: Dict) -> str:
    """Generate Zola TOML frontmatter from Notion properties."""
    # Extract required fields
    title = extract_title_property(properties, "Title") or "Untitled"
    date = extract_date_property(properties, "Publish Date") or datetime.now().strftime("%Y-%m-%d")

    # Extract optional fields
    description = extract_text_property(properties, "SEO Description")
    tags_text = extract_text_property(properties, "Tags")
    category_text = extract_text_property(properties, "Category")

    # Parse tags and categories
    tags = parse_tags_categories(tags_text)
    categories = parse_tags_categories(category_text)

    # Build frontmatter
    frontmatter = "+++\n"
    frontmatter += f'title = "{title}"\n'
    frontmatter += f'date = {date}\n'

    if description:
        # Escape quotes and newlines in description
        description = description.replace('"', '\\"').replace('\n', ' ')
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

    # Split into paragraphs and format
    paragraphs = [p.strip() for p in content.split('\n') if p.strip()]
    return '\n\n'.join(paragraphs) + '\n'

def sync_post(page: Dict) -> bool:
    """Sync a single post from Notion to Zola."""
    properties = page.get("properties", {})

    # Extract status and check if published
    status = extract_text_property(properties, "Status").strip()

    # Try multiple ways to check status
    if not status:
        status = extract_select_property(properties, "Status") or ""

    if status.lower() != "published":
        title = extract_title_property(properties, "Title")
        print(f"  ⊘ Skipping '{title}' (Status: {status})")
        return False

    # Get title and slug
    title = extract_title_property(properties, "Title") or "Untitled"
    slug = extract_text_property(properties, "Slug").strip()

    if not slug:
        slug = slugify(title)

    filename = f"{slug}.md"
    filepath = Path("content/blog") / filename

    print(f"  → Syncing: {title}")

    # Generate frontmatter
    frontmatter = create_frontmatter(properties)

    # Get content from Content property
    content = extract_text_property(properties, "Content")
    formatted_content = format_content(content)

    # Write file
    filepath.parent.mkdir(parents=True, exist_ok=True)
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(frontmatter)
        if formatted_content:
            f.write(formatted_content)
        else:
            f.write("*Content coming soon...*\n")

    print(f"    ✓ Written to {filepath}")
    return True

def main():
    """Main sync function."""
    print("🔄 Starting Notion to Zola sync...\n")

    try:
        # Query published posts
        print("📚 Fetching posts from Notion...")
        posts = query_published_posts()
        print(f"Found {len(posts)} post(s)\n")

        if not posts:
            print("No posts found. Make sure:")
            print("  1. Your NOTION_DATABASE_ID is correct")
            print("  2. Your integration has access to the database")
            print("  3. At least one post exists")
            return

        # Sync each post
        synced = 0
        for post in posts:
            if sync_post(post):
                synced += 1

        print(f"\n✅ Successfully synced {synced} post(s)!")

        if synced > 0:
            print("\nNext steps:")
            print("  1. Run 'zola serve' to preview changes")
            print("  2. Run 'zola build' to build for production")

    except requests.exceptions.HTTPError as e:
        print(f"\n❌ Error communicating with Notion API:")
        print(f"   {e}")
        if hasattr(e.response, 'text'):
            print(f"   Response: {e.response.text}")
        print("\nTroubleshooting:")
        print("  1. Check your NOTION_API_KEY is valid")
        print("  2. Check your NOTION_DATABASE_ID is correct")
        print("  3. Ensure your integration has access to the database")
        print("     (Open database → ... → Add connections → Select your integration)")
    except Exception as e:
        print(f"\n❌ Unexpected error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()
