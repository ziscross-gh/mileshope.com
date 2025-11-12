# Quick Notion Setup Guide

## Your Database Info

**Database ID**: `232ae70a30d480dd9eebe04b46260adf`
**Database Name**: "Mileshope.com"
**Database URL**: https://www.notion.so/232ae70a30d480dd9eebe04b46260adf

---

## Step 1: Share Database with Integration

Your integration needs permission to access the database:

1. **Open your Notion database**:
   - Go to: https://www.notion.so/232ae70a30d480dd9eebe04b46260adf

2. **Click the "..." menu** (top right of the database)

3. **Click "Add connections"**

4. **Select your integration** from the list
   - It should be named something like "Zola Blog Sync" or whatever you named it

5. **Confirm** the connection

---

## Step 2: Verify Your .env File

Your `.env` should have:

```bash
NOTION_API_KEY=secret_your_actual_key_here
NOTION_DATABASE_ID=232ae70a30d480dd9eebe04b46260adf
```

**Note**: The database ID can be with or without dashes - both work!

---

## Step 3: Test the Sync

```bash
# Run sync
set -a && source .env && set +a && python3 sync.py

# You should see:
# 🔄 Starting Notion to Zola sync...
# 📚 Fetching posts from Notion...
# Found X post(s)
# → Syncing: [post title]
# ✓ Written to content/blog/[slug].md
```

---

## Troubleshooting

### Error: "Could not find database"
→ Database not shared with integration (see Step 1)

### Error: "Invalid API key"
→ Check your NOTION_API_KEY in .env

### Error: "No posts found"
→ Make sure at least one post has Status = "Published"

---

## Quick Test

After sharing the database, run:

```bash
set -a && source .env && set +a && python3 sync.py
```

If successful, your posts will appear in `content/blog/`!

Then preview with:
```bash
zola serve
```
