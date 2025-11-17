# Google Analytics 4 Setup Guide

This guide will help you set up Google Analytics 4 (GA4) for MilesHope.com to track visitor behavior, page views, and engagement metrics.

## Why Google Analytics 4?

GA4 provides:
- **User-centric tracking** across devices and platforms
- **Event-based measurement** for better insight into user behavior
- **Privacy-first approach** with built-in IP anonymization
- **Free tier** that handles most small-to-medium website needs
- **Predictive metrics** and AI-powered insights
- **Cross-platform tracking** (web + app if needed later)

## Step 1: Create a Google Analytics 4 Property

1. **Go to Google Analytics**: Visit [analytics.google.com](https://analytics.google.com)

2. **Sign in** with your Google account (or create one if needed)

3. **Create an Account** (if you don't have one):
   - Click "Start measuring"
   - Enter an account name (e.g., "Miles Hope")
   - Configure data sharing settings (optional)
   - Click "Next"

4. **Create a Property**:
   - Property name: "MilesHope.com" or similar
   - Time zone: Select your timezone
   - Currency: Select your preferred currency
   - Click "Next"

5. **About Your Business** (optional):
   - Industry: "Arts & Entertainment" or "Online Communities"
   - Business size: Choose appropriate size
   - How you intend to use Google Analytics: Check relevant boxes
   - Click "Create"

6. **Accept Terms of Service**

## Step 2: Set Up a Data Stream

1. After creating the property, you'll be prompted to set up a data stream
   - Select **Web** as the platform

2. **Configure Web Stream**:
   - Website URL: `https://www.mileshope.com`
   - Stream name: "MilesHope.com Website"
   - Click "Create stream"

3. **Copy Your Measurement ID**:
   - After creating the stream, you'll see a **Measurement ID** in the format `G-XXXXXXXXXX`
   - **Copy this ID** - you'll need it in the next step

## Step 3: Add Measurement ID to Zola Config

1. **Open `config.toml`** in your project

2. **Find the analytics section** (around line 57):
   ```toml
   # Google Analytics 4 - Replace with your actual GA4 Measurement ID (format: G-XXXXXXXXXX)
   # To enable analytics, uncomment the line below and add your Measurement ID
   # google_analytics_id = ""
   ```

3. **Uncomment and add your Measurement ID**:
   ```toml
   # Google Analytics 4
   google_analytics_id = "G-XXXXXXXXXX"  # Replace with your actual ID
   ```

4. **Save the file**

## Step 4: Test the Installation

### Local Testing

1. **Build and serve the site**:
   ```bash
   zola build
   zola serve
   ```

2. **Open your browser** to `http://127.0.0.1:1111`

3. **Open Developer Tools** (F12 or Cmd+Option+I):
   - Go to the **Network** tab
   - Filter for "gtag" or "google-analytics"
   - Reload the page
   - You should see requests to `www.googletagmanager.com/gtag/js`

4. **Check the Console** tab for any errors

### Production Testing (After Deployment)

1. **Visit your live site** at `https://www.mileshope.com`

2. **Use Google Analytics Debugger Chrome Extension** (recommended):
   - Install: [Chrome Web Store - GA Debugger](https://chrome.google.com/webstore/detail/google-analytics-debugger/)
   - Enable the extension
   - Reload your site
   - Check the console for GA debug messages

3. **Check Real-Time Reports** in GA4:
   - Go to Google Analytics dashboard
   - Navigate to Reports → Realtime
   - Visit your website in another tab
   - Within 30 seconds, you should see your visit in the Realtime report

## Step 5: Configure Enhanced Measurement (Optional)

GA4 automatically tracks many interactions, but you can customize:

1. In Google Analytics, go to **Admin** → **Data Streams**
2. Click on your web stream
3. Click **Enhanced measurement**
4. Toggle options on/off:
   - ✅ Page views (automatically tracked)
   - ✅ Scrolls (tracks 90% scroll depth)
   - ✅ Outbound clicks (clicks to external sites)
   - ✅ Site search (if you add search parameter tracking)
   - ✅ Video engagement (if you embed videos)
   - ✅ File downloads (PDF, etc.)

## Privacy Compliance

The implementation includes privacy-friendly settings:

- **IP Anonymization**: Enabled by default with `'anonymize_ip': true`
- **Cookie Flags**: Secure cookies with `'cookie_flags': 'SameSite=None;Secure'`
- **Conditional Loading**: GA4 only loads if `google_analytics_id` is set

### GDPR Compliance (If Needed)

If your site serves EU visitors, consider:

1. **Cookie Consent Banner**: Use a tool like:
   - [Cookiebot](https://www.cookiebot.com/)
   - [OneTrust](https://www.onetrust.com/)
   - Custom implementation

2. **Update Privacy Policy**: Include information about:
   - What data is collected
   - How it's used
   - User rights to opt-out
   - Link to Google's privacy policy

3. **Conditional GA4 Loading**: Only load GA4 after user consent:
   ```javascript
   // Example: Load GA4 only after consent
   if (userHasConsented) {
       gtag('config', 'G-XXXXXXXXXX');
   }
   ```

## Useful Reports to Monitor

Once GA4 is collecting data (takes 24-48 hours for full data):

### Traffic Overview
- **Reports → Life Cycle → Acquisition → Traffic acquisition**
  - See where visitors come from (Google, direct, social, etc.)

### Popular Content
- **Reports → Life Cycle → Engagement → Pages and screens**
  - See which blog posts are most popular

### User Behavior
- **Reports → Life Cycle → Engagement → Events**
  - Track scrolls, clicks, downloads

### Real-Time Activity
- **Reports → Realtime**
  - See current visitors and what they're viewing

## Custom Events (Advanced - Optional)

You can track custom events for deeper insights:

### Example: Track Social Share Clicks

Add this to your page template where social buttons are:

```javascript
// Track when someone clicks a social share button
document.querySelectorAll('.social-share-button').forEach(button => {
    button.addEventListener('click', function(e) {
        const platform = this.dataset.platform; // e.g., "twitter", "linkedin"
        gtag('event', 'share', {
            'method': platform,
            'content_type': 'blog_post',
            'content_id': '{{ page.slug }}'
        });
    });
});
```

### Example: Track Outbound Links

```javascript
// Track clicks to external links
document.querySelectorAll('a[href^="http"]').forEach(link => {
    link.addEventListener('click', function(e) {
        if (!link.href.includes('mileshope.com')) {
            gtag('event', 'click', {
                'event_category': 'outbound',
                'event_label': link.href
            });
        }
    });
});
```

## Troubleshooting

### GA4 Not Tracking Visits

**Check:**
1. Is `google_analytics_id` uncommented in `config.toml`?
2. Did you rebuild the site after adding the ID? (`zola build`)
3. Is the Measurement ID correct (format: `G-XXXXXXXXXX`)?
4. Check browser console for errors
5. Disable ad blockers when testing (they block GA4)

### Data Not Appearing in Reports

**Remember:**
- Real-time reports update within 30 seconds
- Standard reports can take 24-48 hours to populate
- Very low traffic may not show in some aggregated views

### "Property Not Receiving Data" Warning

- This is normal for the first 24-48 hours
- Make sure you've visited the site at least once
- Check that Enhanced Measurement is enabled

## Deployment Checklist

Before deploying to production with GA4 enabled:

- [ ] GA4 property created
- [ ] Measurement ID copied and added to `config.toml`
- [ ] `google_analytics_id` line uncommented
- [ ] Site rebuilt with `zola build`
- [ ] Tested locally (check Network tab for gtag requests)
- [ ] Privacy policy updated (if collecting personal data)
- [ ] Cookie consent implemented (if required by GDPR/CCPA)

## Additional Resources

- [Google Analytics 4 Official Documentation](https://support.google.com/analytics/answer/10089681)
- [GA4 vs Universal Analytics](https://support.google.com/analytics/answer/11583528)
- [GA4 Event Reference](https://support.google.com/analytics/answer/9267735)
- [Privacy Controls in GA4](https://support.google.com/analytics/answer/9019185)

## Removing Google Analytics

If you decide to remove GA4 later:

1. Comment out the line in `config.toml`:
   ```toml
   # google_analytics_id = "G-XXXXXXXXXX"
   ```

2. Rebuild: `zola build`

The conditional `{% if config.extra.google_analytics_id %}` in `base.html` ensures the tracking script won't load when the ID is commented out or removed.

---

**Need help with GA4 setup?** Check the [Google Analytics Help Center](https://support.google.com/analytics) or search for specific questions on the GA4 community forums.
