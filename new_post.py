import argparse
import os
from datetime import datetime
import re

def slugify(text):
    """Convert text to a slug."""
    text = text.lower()
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '-', text)
    return text.strip('-')

def create_post(title, category=None, tags=None):
    """Create a new blog post with frontmatter."""
    date_str = datetime.now().strftime('%Y-%m-%d')
    slug = slugify(title)
    filename = f"content/blog/{slug}.md"
    
    if os.path.exists(filename):
        print(f"❌ Error: File '{filename}' already exists.")
        return

    # Format tags and categories for TOML
    tags_str = str(tags).replace("'", '"') if tags else '[]'
    categories_str = f'["{category}"]' if category else '[]'

    content = f"""+++
title = "{title}"
date = {date_str}
description = "Enter description here"

[taxonomies]
categories = {categories_str}
tags = {tags_str}
+++

Write your content here...
"""

    try:
        with open(filename, 'w') as f:
            f.write(content)
        print(f"✅ Created new post: {filename}")
    except Exception as e:
        print(f"❌ Error creating file: {e}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Scaffold a new blog post.")
    parser.add_argument("title", help="Title of the blog post")
    parser.add_argument("-c", "--category", help="Category for the post")
    parser.add_argument("-t", "--tags", nargs="+", help="Tags for the post")
    
    args = parser.parse_args()
    
    create_post(args.title, args.category, args.tags)
