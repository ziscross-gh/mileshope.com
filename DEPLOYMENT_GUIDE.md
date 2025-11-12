# Cloudflare Pages Deployment Guide

This guide will help you deploy MilesHope.com to Cloudflare Pages.

## Prerequisites

- ✅ Zola site built and tested locally
- ✅ Git repository with all changes committed
- ✅ GitHub account (or GitLab/Bitbucket)
- ✅ Cloudflare account (free tier works great)
- ✅ Domain name (mileshope.com)

## Step 1: Push to GitHub

If you haven't already pushed your code to GitHub:

```bash
# Create a new repository on GitHub (https://github.com/new)
# Name it: mileshope.com

# Add remote (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/mileshope.com.git

# Push your code
git push -u origin main
```

## Step 2: Set Up Cloudflare Pages

### 2.1 Connect Your Repository

1. Go to [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. Navigate to **Workers & Pages** → **Create application** → **Pages**
3. Click **Connect to Git**
4. Authorize Cloudflare to access your GitHub account
5. Select your `mileshope.com` repository
6. Click **Begin setup**

### 2.2 Configure Build Settings

Use these exact settings:

| Setting | Value |
|---------|-------|
| **Project name** | `mileshope` |
| **Production branch** | `main` |
| **Framework preset** | `Zola` |
| **Build command** | `zola build` |
| **Build output directory** | `public` |

**Environment Variables:** (if needed)
- None required for basic Zola deployment

### 2.3 Deploy

1. Click **Save and Deploy**
2. Cloudflare will:
   - Clone your repository
   - Install Zola automatically
   - Run `zola build`
   - Deploy the `public/` folder
3. Wait 2-3 minutes for the first build
4. You'll get a URL like: `https://mileshope.pages.dev`

## Step 3: Custom Domain Setup

### 3.1 Add Custom Domain

1. In your Cloudflare Pages project, go to **Custom domains**
2. Click **Set up a custom domain**
3. Enter: `mileshope.com`
4. Click **Continue**

### 3.2 Configure DNS

Cloudflare will provide DNS records. You need to:

**Option A: If your domain is already on Cloudflare:**
- Records will be added automatically ✨
- Just wait 2-3 minutes

**Option B: If your domain is elsewhere:**
1. Add these records at your domain registrar:

```
CNAME  mileshope.com  →  mileshope.pages.dev
```

2. Or migrate your domain to Cloudflare (recommended):
   - Go to **Websites** → **Add a site**
   - Enter `mileshope.com`
   - Follow the nameserver setup instructions
   - Update nameservers at your registrar

### 3.3 Add www Subdomain (Optional)

1. Click **Set up a custom domain** again
2. Enter: `www.mileshope.com`
3. Set up redirect: `www.mileshope.com` → `mileshope.com`

## Step 4: SSL/HTTPS Configuration

Cloudflare automatically provides:
- ✅ Free SSL certificate
- ✅ Automatic HTTPS redirects
- ✅ HTTP/2 and HTTP/3 support

No configuration needed! 🎉

## Step 5: Automatic Deployments

Once set up, deployments are automatic:

1. **Push to GitHub:**
   ```bash
   git add .
   git commit -m "Update content"
   git push origin main
   ```

2. **Cloudflare automatically:**
   - Detects the push
   - Runs `zola build`
   - Deploys the new version
   - Usually takes 1-2 minutes

3. **View deployment:**
   - Go to your Cloudflare Pages project
   - See deployment history and logs

## Step 6: Performance Optimization

Cloudflare Pages includes these optimizations automatically:

- ✅ **Global CDN** - 300+ edge locations worldwide
- ✅ **Auto Minify** - CSS, JS, and HTML compression
- ✅ **Brotli Compression** - Better than gzip
- ✅ **HTTP/3 + QUIC** - Latest protocols
- ✅ **Smart Caching** - Cache headers optimized
- ✅ **DDoS Protection** - Enterprise-grade security

### Additional Optimizations (Optional)

In Cloudflare Dashboard → **Speed** → **Optimization**:

1. **Enable:**
   - Auto Minify (HTML, CSS, JS)
   - Brotli compression
   - Early Hints
   - Rocket Loader (for JS)

2. **Caching:**
   - Already optimized for static sites
   - No changes needed

## Step 7: Environment Variables (For Notion Sync)

If you want to sync Notion content directly in CI/CD:

1. Go to **Settings** → **Environment variables**
2. Add variables:
   - `NOTION_API_KEY` = your_key_here
   - `NOTION_DATABASE_ID` = your_database_id

3. Update build command:
   ```bash
   python3 sync.py && zola build
   ```

Note: You'll need to commit `sync.py` and ensure Python 3 is available in the build environment.

## Troubleshooting

### Build Fails

**Problem:** Zola version mismatch
**Solution:** Cloudflare uses latest stable Zola. Test locally with:
```bash
zola --version
```

**Problem:** Build command not found
**Solution:** Double-check build settings:
- Build command: `zola build`
- Output directory: `public`

### Custom Domain Not Working

**Problem:** DNS not propagating
**Solution:** Wait up to 24 hours, check with:
```bash
dig mileshope.com
```

**Problem:** SSL certificate pending
**Solution:** Cloudflare provisions certificates automatically. Wait 10-15 minutes.

### Site Not Updating

**Problem:** Cache not clearing
**Solution:**
1. In Cloudflare: **Caching** → **Purge Everything**
2. Hard refresh browser: `Cmd+Shift+R` (Mac) or `Ctrl+F5` (Windows)

## Monitoring & Analytics

### Cloudflare Analytics (Built-in)

1. Go to **Analytics** in Cloudflare Pages
2. View:
   - Page views
   - Unique visitors
   - Bandwidth usage
   - Geographic distribution

### Add Google Analytics (Optional)

1. Get GA4 tracking ID
2. Add to `templates/base.html`:
   ```html
   <!-- Google Analytics -->
   <script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
   <script>
     window.dataLayer = window.dataLayer || [];
     function gtag(){dataLayer.push(arguments);}
     gtag('js', new Date());
     gtag('config', 'G-XXXXXXXXXX');
   </script>
   ```

## Performance Targets

After deployment, you should see:

- ⚡ **First Contentful Paint** < 1s
- ⚡ **Time to Interactive** < 2s
- ⚡ **Lighthouse Score** > 95
- ⚡ **Global load time** < 500ms

Test with: [PageSpeed Insights](https://pagespeed.web.dev/)

## Deployment Checklist

Before going live:

- [ ] All content reviewed and proofread
- [ ] About and Services pages complete
- [ ] At least 3-5 blog posts published
- [ ] All images optimized
- [ ] Meta descriptions added
- [ ] Social sharing tested
- [ ] Mobile responsive verified
- [ ] Search functionality working
- [ ] Dark mode tested
- [ ] 404 page styled
- [ ] RSS feed validated

## Post-Deployment Tasks

After deployment:

1. **Submit to Search Engines:**
   - Google Search Console
   - Bing Webmaster Tools

2. **Create Sitemap:**
   - Zola auto-generates at `/sitemap.xml`
   - Submit to Google Search Console

3. **Verify:**
   - Check all pages load correctly
   - Test all links
   - Verify RSS feed: `/rss.xml`
   - Test search functionality
   - Check mobile responsiveness

4. **Promote:**
   - Share on social media
   - Update email signature
   - Add to online profiles

## Costs

- **Cloudflare Pages**: FREE
  - Unlimited requests
  - Unlimited bandwidth
  - 500 builds/month (free tier)
  - Custom domains included

- **Domain**: ~$10-15/year (from registrar)

Total monthly cost: **$0** (after domain purchase) 🎉

## Support

- **Cloudflare Pages Docs**: https://developers.cloudflare.com/pages/
- **Zola Docs**: https://www.getzola.org/documentation/
- **Community**: Cloudflare Discord & Forum

---

## Quick Reference Commands

```bash
# Local development
zola serve

# Build for production
zola build

# Check for errors
zola check

# Sync from Notion
source .env && python3 sync.py

# Deploy (automatic via git push)
git add .
git commit -m "Update content"
git push origin main
```

---

**Ready to deploy?** Follow the steps above and your site will be live in minutes! 🚀
