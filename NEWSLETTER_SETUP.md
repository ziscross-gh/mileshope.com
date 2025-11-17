# Newsletter Setup Guide

Your site now includes beautiful newsletter signup forms on the homepage and at the end of every blog post. Follow this guide to connect them to your email service provider.

## Newsletter Locations

1. **Homepage** - Prominent signup between hero and recent posts
2. **Blog Posts** - End-of-post signup after author bio

## Quick Setup

### 1. Choose Your Email Service Provider

Popular options for content creators:

- **ConvertKit** - Best for creators, easy automation
- **Mailchimp** - Free up to 500 subscribers
- **Substack** - Built-in newsletter platform
- **EmailOctopus** - Cost-effective, simple
- **Buttondown** - Minimalist, markdown-friendly

### 2. Get Your Form Action URL

Each provider gives you a form submission URL. Here's where to find it:

#### ConvertKit
1. Create a form at https://app.convertkit.com/forms
2. Go to "Embed" → "HTML"
3. Look for the `<form action="...">` URL
4. Example: `https://app.convertkit.com/forms/YOUR_FORM_ID/subscriptions`

#### Mailchimp
1. Create a form/audience
2. Go to "Signup forms" → "Embedded forms"
3. Find the form action URL in the HTML
4. Example: `https://YOUR_USERNAME.usX.list-manage.com/subscribe/post?u=USER_ID&id=LIST_ID`

#### Substack
1. Your publication URL + `/api/v1/free?nojs=true`
2. Example: `https://yourname.substack.com/api/v1/free?nojs=true`

#### EmailOctopus
1. Create a list
2. Go to "Forms" → "Embed code"
3. Extract the action URL
4. Example: `https://emailoctopus.com/lists/YOUR_LIST_ID/members`

### 3. Update the Newsletter Template

Edit `templates/macros/newsletter.html`:

Find this line in each macro (around lines 22, 52, 85):
```html
<form class="newsletter-form" method="post" action="#newsletter-form" id="newsletterForm">
```

Replace `#newsletter-form` with your provider's URL:
```html
<form class="newsletter-form" method="post" action="YOUR_PROVIDER_URL_HERE" id="newsletterForm">
```

**Important:** Update the action URL in ALL THREE form locations:
- `inline()` macro (homepage)
- `popup()` macro (future use)
- `post_end()` macro (blog posts)

### 4. Update Hidden Fields (if needed)

Some providers require additional hidden fields. Add them inside the `<form>` tag after the email input:

**ConvertKit:**
```html
<input type="hidden" name="utf8" value="✓">
```

**Mailchimp:**
```html
<input type="hidden" name="b_USER_ID_LIST_ID" value="">
```

**Substack:**
```html
<input type="hidden" name="first_url" value="{{ page.permalink | default(value=config.base_url) }}">
```

### 5. Test Your Signup

1. Build your site: `zola build`
2. Test locally: `zola serve`
3. Visit http://127.0.0.1:1111
4. Try subscribing with a test email
5. Check your email service provider dashboard to confirm the subscription

### 6. Deploy

Once tested:
```bash
git add templates/macros/newsletter.html
git commit -m "Connect newsletter to [Your Provider]"
git push origin main
```

## Advanced: Custom Success/Error Handling

For a better user experience, add JavaScript to handle form submissions:

Create `static/js/newsletter.js`:

```javascript
document.addEventListener('DOMContentLoaded', function() {
    const forms = document.querySelectorAll('.newsletter-form');

    forms.forEach(form => {
        form.addEventListener('submit', async function(e) {
            e.preventDefault();

            const email = this.querySelector('input[type="email"]').value;
            const submitBtn = this.querySelector('.newsletter-submit');
            const response = this.querySelector('.newsletter-response');

            // Disable button during submission
            submitBtn.disabled = true;
            submitBtn.textContent = 'Subscribing...';

            try {
                // Submit to your provider
                const result = await fetch(this.action, {
                    method: 'POST',
                    body: new FormData(this),
                    mode: 'no-cors' // Required for cross-origin
                });

                // Show success message
                response.className = 'newsletter-response success';
                response.textContent = 'Success! Check your email to confirm.';
                this.querySelector('input[type="email"]').value = '';

            } catch (error) {
                // Show error message
                response.className = 'newsletter-response error';
                response.textContent = 'Something went wrong. Please try again.';
            } finally {
                // Re-enable button
                submitBtn.disabled = false;
                submitBtn.textContent = 'Subscribe';
            }
        });
    });
});
```

Then add to `templates/base.html` before `</body>`:
```html
<script src="{{ get_url(path='js/newsletter.js') }}"></script>
```

## Styling Customization

Newsletter styles are in `styles/input.css` starting at line ~1600.

Key customization points:

**Colors:**
- `.newsletter-submit` background (line ~1672)
- `.newsletter-inline` gradient (line ~1713)

**Sizing:**
- `.newsletter-content` max-width (line ~1609)
- `.newsletter-title` font-size (line ~1611)

**Dark Mode:**
All components have dark mode support via `html.dark` selectors.

## Tracking & Analytics

### Google Analytics 4

Track newsletter signups as events:

```javascript
// Add to form submit handler
gtag('event', 'newsletter_signup', {
  'event_category': 'engagement',
  'event_label': 'newsletter'
});
```

### ConvertKit

Built-in analytics in dashboard.

### Mailchimp

Reports available under "Audience" → "Signup forms"

## Best Practices

1. **Welcome Email** - Set up an automated welcome series
2. **Lead Magnet** - Offer a free resource (Bazi guide, Tarot spread PDF)
3. **Consistency** - Send regularly (weekly recommended)
4. **Value First** - Don't just promote; provide insights
5. **Segment** - Tag subscribers by interest (Bazi, Tarot, both)

## Troubleshooting

**Form doesn't submit:**
- Check browser console for errors
- Verify the action URL is correct
- Ensure CORS is configured on your provider

**Subscribers not appearing:**
- Confirm email address in provider dashboard
- Check spam folder for confirmation email
- Verify hidden fields match provider requirements

**Styling issues:**
- Run `zola build` to recompile CSS
- Clear browser cache
- Check dark mode styles

## Next Steps

Once your newsletter is set up:

1. **Create welcome sequence** - 3-5 emails introducing new subscribers to your content
2. **Plan content calendar** - What will you send weekly?
3. **Build lead magnet** - Free PDF guide to boost signups
4. **Add exit-intent popup** - Capture leaving visitors (template included but disabled)
5. **A/B test copy** - Try different headlines/descriptions

## Support

- ConvertKit: https://help.convertkit.com
- Mailchimp: https://mailchimp.com/help
- Substack: https://support.substack.com
- EmailOctopus: https://help.emailoctopus.com

---

**Need Help?** If you're stuck, most providers offer free setup calls or have detailed documentation for custom HTML forms.
