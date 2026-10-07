# MilesHope.com on EmDash

The new MilesHope.com: Thai tarot (Promayarn cards) and Thai astrology consultations, with articles under Understand, Build and Apply. Built with [EmDash](https://emdashcms.com) on Astro.

This is the `main` branch. The previous Zola site, with its posts and Notion sync script, is kept on the `zola-archive` branch.

The site is live at `www.mileshope.com`. Every push to `main` is built and deployed by Cloudflare.

## Run it on your computer

You need Node.js 22 or newer and pnpm.

```bash
pnpm install
pnpm dev
```

1. Open http://localhost:4321/_emdash/admin and finish the short setup. You register your own passkey there. Keep the option to include the starting content switched on, so the pages are filled from `seed/seed.json`.
2. Open http://localhost:4321 to see the site.

`pnpm dev` runs a local copy of the Cloudflare runtime. Its database lives in the `.wrangler` folder. To start again from the original content, stop the server, delete `.wrangler`, and run setup again.

## Cloudflare

The site runs as a Cloudflare Worker named `mileshope-com`, set up in `wrangler.jsonc`, and is live at `www.mileshope.com`:

- **Database:** the D1 database `mileshope` (already created, Asia-Pacific).
- **Sessions:** a KV namespace that Cloudflare creates on the first deploy.
- **Images:** the R2 bucket `mileshope-media` (Standard storage, Asia-Pacific).
- **Domain:** `www.mileshope.com` is attached to the Worker in the Cloudflare dashboard, not in `wrangler.jsonc`. The `workers.dev` address is switched off.

To deploy from your computer:

```bash
pnpm wrangler login
pnpm deploy
```

Or let Cloudflare build it from GitHub. In the Cloudflare dashboard go to Workers & Pages, Create application, Import a repository, pick this repository and set:

| Setting | Value |
|---|---|
| Project name | `mileshope-com` (must match `name` in `wrangler.jsonc`) |
| Branch | `main` |
| Build command | `pnpm build` |
| Deploy command | `npx wrangler deploy` |

Then turn off builds for other branches on that Worker, so pushes to `zola-archive` (the old Zola site) do not start builds there. Node.js is pinned to version 22 by `.node-version`, and pnpm picks its own version from `package.json`.

The admin is at `https://www.mileshope.com/_emdash/admin`. Setup has been run there, so passkeys are registered on that address.

## What you can edit in the admin

| In the admin | What it changes |
|---|---|
| Posts | Articles. Each has a Section (Understand, Build, Apply, Notes) and Topics. Tick "Featured" to put one in the large card on the home page. |
| Pages | Simple pages such as About. |
| Sessions | The consultation cards: name, length, price, description, button, order. |
| Home page | Every heading, paragraph and button label on the home page. |
| Consultations page | Every heading, the steps, the two lists, and the WhatsApp and LINE details. |
| Menus | The links in the header and footer. |
| Settings | Site title and tagline. |

When you are signed in, you can also click text on the site itself to edit it in place.

## Still to fill in

Search the admin for square brackets. These are waiting for real details:

- `[YOUR WHATSAPP NUMBER]` and `[YOUR LINE ID]`, plus their links (Consultations page)
- `[PAYMENT METHOD]` (Consultations page, step 2)
- `[PRICE]` and `[VENUE OR AREA]` (Sessions, Face to face)
- The About story (Pages, About, and the About block on the Home page)
- Four posts are outlines only, and the Promayarn article is a draft to check against your own practice.

## Not built yet

- Thai versions of the pages (planned under `/th/`)
- A contact form

## Where things are

- `seed/seed.json`: the content model and starting content
- `wrangler.jsonc`, `astro.config.mjs`: Cloudflare and EmDash setup
- `src/pages/`: one file per page type
- `src/layouts/Base.astro`: header and footer
- `src/styles/theme.css`: brand colours, type and shared styles
- `public/brand/`, `public/fonts/`: logo, zodiac badges, fonts
