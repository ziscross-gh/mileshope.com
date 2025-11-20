# Deployment Guide

Complete guide for deploying MilesHope.com to Cloudflare Pages.

## Overview

MilesHope.com is deployed on **Cloudflare Pages** with automatic deployments from GitHub.

**Deployment Flow:**
```
Git Push -> GitHub -> Cloudflare Pages -> Build -> Deploy -> CDN
```

## Prerequisites

- GitHub account
- Cloudflare account
- Domain (optional, Cloudflare provides subdomain)

## Initial Setup

### 1. Push to GitHub

```bash
# If not already done
git remote add origin https://github.com/ziscross-gh/mileshope.com.git
git branch -M main
git push -u origin main
```

### 2. Connect to Cloudflare Pages

1. Log in to [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. Select "Workers & Pages" from sidebar
3. Click "Create application"
4. Select "Pages" tab
5. Click "Connect to Git"

### 3. Configure Repository

1. **Select repository:** `ziscross-gh/mileshope.com`
2. Click "Begin setup"

### 4. Build Configuration

**Project name:** `mileshope`

**Production branch:** `main`

**Build settings:**
- Framework preset: `None`
- Build command: `./build.sh`
- Build output directory: `public`
- Root directory: `/` (leave empty)

**Environment variables:** None required

Click "Save and Deploy"

### 5. First Deployment

Cloudflare will:
1. Clone your repository
2. Run `./build.sh`
3. Deploy `public/` directory
4. Provide URL: `https://mileshope.pages.dev`

Wait 2-5 minutes for first build.

## Custom Domain Setup

### Option 1: Cloudflare-Managed Domain

If domain is already on Cloudflare:

1. Go to your Pages project
2. Click "Custom domains"
3. Click "Set up a custom domain"
4. Enter `www.mileshope.com`
5. Click "Activate domain"

DNS records are added automatically.

### Option 2: External Domain

If domain is elsewhere:

1. Get Cloudflare DNS records from Pages dashboard
2. Add CNAME record to your DNS provider:
   ```
   www.mileshope.com CNAME mileshope.pages.dev
   ```
3. Wait for DNS propagation (5-30 minutes)

### SSL Certificate

Cloudflare automatically provisions SSL certificates:
- Universal SSL (free)
- Auto-renews
- HTTPS enforced

## Deployment Workflow

### Automatic Deployments

Every push to `main` triggers deployment:

```bash
git add .
git commit -m "Update content"
git push origin main
```

Cloudflare automatically:
1. Detects push
2. Starts build
3. Runs `./build.sh`
4. Deploys if successful
5. Sends notification (if configured)

**Build time:** 30-60 seconds

### Preview Deployments

Every pull request creates a preview:

```bash
git checkout -b feature/new-post
# Make changes
git push origin feature/new-post
# Create PR on GitHub
```

Preview URL: `https://UNIQUE_ID.mileshope.pages.dev`

### Manual Deployments

To manually trigger deployment:

1. Go to Cloudflare Pages dashboard
2. Select your project
3. Click "Deployments"
4. Click "Retry deployment" on any build

## Build Process

### What Happens During Build

1. **Clone repository**
   ```bash
   git clone https://github.com/ziscross-gh/mileshope.com.git
   cd mileshope.com
   ```

2. **Run build script**
   ```bash
   chmod +x build.sh
   ./build.sh
   ```

3. **Build steps** (inside `build.sh`):
   ```bash
   # Compile Tailwind CSS
   ./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify

   # Build Zola site
   zola build
   ```

4. **Deploy `public/` directory**

### Build Environment

- **OS:** Ubuntu Linux
- **Shell:** Bash
- **Node.js:** Not required (we use standalone binaries)
- **Rust:** Pre-installed (for Zola if needed)

### Build Logs

View logs in Cloudflare dashboard:
1. Go to your project
2. Click "Deployments"
3. Click any deployment
4. View "Build log" tab

## Troubleshooting

### Build Fails

**Error:** `Permission denied: ./build.sh`

**Solution:**
```bash
git update-index --chmod=+x build.sh
git commit -m "Make build.sh executable"
git push origin main
```

---

**Error:** `./tailwindcss: not found`

**Solution:**
Ensure `tailwindcss` binary is committed:
```bash
git add tailwindcss
git commit -m "Add Tailwind CSS binary"
git push origin main
```

---

**Error:** `zola: command not found`

**Solution:** Zola is pre-installed on Cloudflare Pages. If missing, add to build command:
```bash
# In Cloudflare Pages settings, change build command to:
curl -sL https://github.com/getzola/zola/releases/download/v0.18.0/zola-v0.18.0-x86_64-unknown-linux-gnu.tar.gz | tar xz && ./build.sh
```

### Deployment Succeeds but Site Broken

**Issue:** CSS not loading

**Solution:**
1. Check `static/css/tailwind.css` is committed
2. Verify `base_url` in `config.toml` matches your domain
3. Hard refresh browser (`Cmd+Shift+R`)

---

**Issue:** Pages show 404

**Solution:**
1. Check `public/` directory structure
2. Verify Zola build completed successfully
3. Check build logs for errors

### Slow Deployments

**Normal:** 30-60 seconds

**Slow (2-3 minutes):**
- Large content files
- Many images
- First build after long time

**Very slow (5+ minutes):**
- Check build logs for hanging process
- Retry deployment

### Domain Not Working

**Issue:** Custom domain shows error

**Solution:**
1. Verify DNS records are correct
2. Wait for DNS propagation (up to 24 hours)
3. Check SSL certificate status
4. Try accessing via Pages subdomain first

## Environment Variables

Currently, no environment variables are needed for production.

If you add features requiring secrets:

1. Go to Cloudflare Pages dashboard
2. Select your project
3. Go to "Settings" > "Environment variables"
4. Add variables
5. Redeploy

**Example:**
```
Name: NOTION_API_KEY
Value: secret_xxxxx
Environments: Production
```

## Monitoring

### Cloudflare Analytics

1. Go to your project dashboard
2. Click "Analytics" tab
3. View:
   - Page views
   - Requests
   - Bandwidth
   - Geographic distribution

### Build Notifications

Configure in Cloudflare Pages:
1. Go to "Settings"
2. Click "Notifications"
3. Add email or webhook
4. Select events:
   - Deployment success
   - Deployment failure

### Uptime Monitoring

Free options:
- [UptimeRobot](https://uptimerobot.com/)
- [Pingdom](https://www.pingdom.com/)
- [StatusCake](https://www.statuscake.com/)

## Performance Optimization

### Cloudflare Settings

1. **Auto Minify**
   - Go to Speed > Optimization
   - Enable: HTML, CSS, JavaScript

2. **Brotli Compression**
   - Enabled by default
   - No configuration needed

3. **Caching**
   - CDN caching automatic
   - No configuration needed

4. **HTTP/3**
   - Go to Network
   - Enable HTTP/3 (with QUIC)

### DNS Settings

1. **Proxied Records** (orange cloud)
   - Enables CDN
   - Recommended for all records

2. **DNSSEC**
   - Go to DNS > Settings
   - Enable DNSSEC

## Rollback

### To Previous Deployment

1. Go to "Deployments" tab
2. Find successful previous build
3. Click "..." menu
4. Click "Rollback to this deployment"

### To Specific Commit

```bash
git revert HEAD
git push origin main
# Or
git reset --hard <commit-hash>
git push --force origin main  # Use with caution
```

## Branch Deployments

### Production (main)

- URL: `https://www.mileshope.com`
- Auto-deploys on push to `main`

### Staging (Optional)

Create staging branch:
```bash
git checkout -b staging
git push origin staging
```

Configure in Cloudflare:
1. Go to Settings > Builds & deployments
2. Set "Production branch" to `main`
3. Set "Preview branch" to `staging`

- Staging URL: `https://staging.mileshope.pages.dev`

## Security

### HTTPS Only

Enabled by default:
- All HTTP requests redirect to HTTPS
- HSTS header sent
- TLS 1.2+ only

### Headers

Configure security headers:
1. Create `_headers` file in `static/`
2. Add headers:
   ```
   /*
     X-Frame-Options: DENY
     X-Content-Type-Options: nosniff
     X-XSS-Protection: 1; mode=block
     Referrer-Policy: strict-origin-when-cross-origin
   ```
3. Commit and deploy

### DDoS Protection

Cloudflare provides:
- Automatic DDoS mitigation
- Rate limiting
- Bot protection
- No configuration needed

## Costs

### Cloudflare Pages Pricing

**Free tier includes:**
- Unlimited requests
- Unlimited bandwidth
- 500 builds per month
- 1 build at a time

**Sufficient for:**
- Personal blogs
- Portfolio sites
- Documentation sites

**Paid tier ($20/month):**
- 5,000 builds per month
- 5 concurrent builds
- Advanced features

**Current usage:** Free tier sufficient

## Backup Strategy

### Git Repository

- Source of truth: GitHub
- All content versioned
- Full history preserved

### Manual Backup

```bash
# Clone repository
git clone https://github.com/ziscross-gh/mileshope.com.git backup

# Or download as ZIP
# GitHub > Code > Download ZIP
```

### Automated Backup

Not needed - Git serves as backup.

## Disaster Recovery

### Site Down

1. Check [Cloudflare Status](https://www.cloudflarestatus.com/)
2. Check build logs
3. Rollback to previous deployment
4. Contact Cloudflare support

### Repository Lost

1. Clone from Cloudflare deployment
2. Extract from backup
3. Restore from local copy

### Domain Hijacked

1. Lock domain at registrar
2. Enable 2FA on all accounts
3. Contact Cloudflare support

## Resources

- [Cloudflare Pages Docs](https://developers.cloudflare.com/pages/)
- [Cloudflare Community](https://community.cloudflare.com/)
- [Zola Deployment Guide](https://www.getzola.org/documentation/deployment/cloudflare-pages/)

---

**Last Updated:** January 2025
