# MilesHope.com - Deployment Checklist

Use this checklist to deploy your site to production.

## Pre-Deployment ✅

- [x] Site builds successfully (`zola build`)
- [x] No broken links (`zola check`)
- [x] All content reviewed
- [x] Dark mode tested
- [x] Mobile responsive verified
- [x] Search functionality working
- [x] Social sharing tested
- [x] All changes committed to git

## GitHub Setup 📦

- [ ] Create GitHub repository: `mileshope.com`
- [ ] Add remote: `git remote add origin https://github.com/YOUR_USERNAME/mileshope.com.git`
- [ ] Push code: `git push -u origin main`
- [ ] Verify repository on GitHub

## Cloudflare Pages Setup ☁️

### Initial Setup
- [ ] Log into [Cloudflare Dashboard](https://dash.cloudflare.com/)
- [ ] Navigate to **Workers & Pages** → **Create application** → **Pages**
- [ ] Click **Connect to Git**
- [ ] Authorize GitHub access
- [ ] Select `mileshope.com` repository

### Build Configuration
- [ ] Project name: `mileshope`
- [ ] Production branch: `main`
- [ ] Framework preset: `Zola`
- [ ] Build command: `zola build`
- [ ] Build output directory: `public`
- [ ] Click **Save and Deploy**

### First Deployment
- [ ] Wait for build to complete (2-3 minutes)
- [ ] Verify site at: `https://mileshope.pages.dev`
- [ ] Test homepage loads
- [ ] Test blog page
- [ ] Test search functionality
- [ ] Test dark mode toggle

## Custom Domain Setup 🌐

### Add Domain
- [ ] In Cloudflare Pages, go to **Custom domains**
- [ ] Click **Set up a custom domain**
- [ ] Enter: `www.mileshope.com` (primary)
- [ ] Click **Continue**

### DNS Configuration

**Option A: Domain already on Cloudflare**
- [ ] Records added automatically
- [ ] Wait 2-3 minutes for propagation

**Option B: Domain elsewhere**
- [ ] Add CNAME record: `www` → `mileshope.pages.dev`
- [ ] OR migrate to Cloudflare (recommended)
- [ ] Wait up to 24 hours for DNS propagation

### Root Domain Redirect (Optional)
- [ ] Add custom domain: `mileshope.com` (without www)
- [ ] Configure redirect: `mileshope.com` → `www.mileshope.com`

### SSL Certificate
- [ ] Verify SSL certificate is active (automatic)
- [ ] Test HTTPS: `https://www.mileshope.com`

## Post-Deployment Verification ✓

### Functionality Tests
- [ ] Homepage loads correctly
- [ ] Blog listing shows all posts
- [ ] Individual blog posts load
- [ ] About page displays properly
- [ ] Services page displays properly
- [ ] Search functionality works
- [ ] Dark mode toggle works
- [ ] Mobile navigation works
- [ ] Social sharing buttons work
- [ ] Copy link works
- [ ] 404 page displays

### Performance Tests
- [ ] Run [PageSpeed Insights](https://pagespeed.web.dev/)
- [ ] Target: Score > 95
- [ ] Check load time < 2s
- [ ] Test from different locations

### SEO Setup
- [ ] Verify sitemap: `/sitemap.xml`
- [ ] Verify RSS feed: `/rss.xml`
- [ ] Submit to [Google Search Console](https://search.google.com/search-console/)
- [ ] Submit to [Bing Webmaster Tools](https://www.bing.com/webmasters/)

## Optional Enhancements 🎯

### Analytics
- [ ] Set up Google Analytics 4
- [ ] Add tracking code to `templates/base.html`
- [ ] Verify tracking in GA4 dashboard

### Monitoring
- [ ] Enable Cloudflare Web Analytics (free)
- [ ] Set up uptime monitoring (optional)
- [ ] Configure error notifications

### Optimization
- [ ] Enable Auto Minify (HTML, CSS, JS)
- [ ] Enable Brotli compression
- [ ] Review Cloudflare Speed settings

## Continuous Deployment 🔄

Once deployed, updates are automatic:

```bash
# Make changes locally
zola serve  # Test changes

# Commit and push
git add .
git commit -m "Update content"
git push origin main

# Cloudflare automatically:
# 1. Detects the push
# 2. Runs zola build
# 3. Deploys the new version
# 4. Usually takes 1-2 minutes
```

## Notion Content Sync 📝

To sync new blog posts from Notion:

```bash
# Sync from Notion
source .env
python3 sync.py

# Review changes
zola serve

# Commit and deploy
git add content/blog/
git commit -m "Add new blog posts"
git push origin main
```

## Troubleshooting 🔧

### Build Fails
- [ ] Check build logs in Cloudflare Pages
- [ ] Verify `zola build` works locally
- [ ] Check Zola version compatibility

### Domain Not Working
- [ ] Verify DNS records are correct
- [ ] Wait for DNS propagation (up to 24h)
- [ ] Check domain status in Cloudflare

### Site Not Updating
- [ ] Purge Cloudflare cache
- [ ] Hard refresh browser (Cmd+Shift+R)
- [ ] Check deployment status in Cloudflare

### SSL Issues
- [ ] Wait 10-15 minutes for certificate provisioning
- [ ] Verify domain is pointed correctly
- [ ] Check SSL/TLS settings in Cloudflare

## Support Resources 📚

- **Deployment Guide**: See `DEPLOYMENT_GUIDE.md`
- **Cloudflare Docs**: https://developers.cloudflare.com/pages/
- **Zola Docs**: https://www.getzola.org/documentation/
- **Notion Sync**: See `SYNC_README.md`

---

## Quick Commands Reference

```bash
# Local development
zola serve                    # Start dev server

# Build
zola build                    # Build for production
zola check                    # Check for errors

# Git
git status                    # Check status
git add .                     # Stage changes
git commit -m "message"       # Commit
git push origin main          # Deploy

# Notion sync
source .env                   # Load environment
python3 sync.py              # Sync content
```

---

**Status**: Ready for deployment! 🚀

**Estimated time**: 15-30 minutes (first time)

**Cost**: FREE (Cloudflare Pages) + domain ($10-15/year)
