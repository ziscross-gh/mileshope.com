import cloudflare from "@astrojs/cloudflare";
import react from "@astrojs/react";
import { d1, r2 } from "@emdash-cms/cloudflare";
import { defineConfig } from "astro/config";
import emdash from "emdash/astro";

// Old addresses from the previous (Zola) site, sent to the closest page on
// this one. 301 where there is a clear new home, 302 where the match is only
// approximate (old posts, tags and categories), so those can change later.
// Any other address under /blog, /tags or /categories goes to the list of all
// articles; see the [...path].astro pages in src/pages.
const temporary = (destination) => ({ status: 302, destination });
const oldSiteRedirects = {
	"/services": "/consultations",
	"/contact": "/consultations#book",
	"/blog": "/posts",

	// Old tarot articles: the Thai tarot topic page.
	"/blog/understanding-tarot-beginners-guide": temporary("/tag/thai-tarot"),
	"/blog/tarot-spreads-guide": temporary("/tag/thai-tarot"),
	"/blog/reading-tarot-intuitively-vs-by-the-book": temporary("/tag/thai-tarot"),
	"/blog/how-to-choose-tarot-vs-bazi": temporary("/tag/thai-tarot"),
	// The piece on technology and spirituality: the About page.
	"/blog/technology-and-spirituality-intersection": temporary("/about"),
	// The two welcome posts: the home page.
	"/blog/welcome": temporary("/"),
	"/blog/welcome-to-mileshope": temporary("/"),

	"/tags": temporary("/posts"),
	"/tags/tarot": temporary("/tag/thai-tarot"),
	"/categories": temporary("/posts"),
	"/categories/tarot": temporary("/tag/thai-tarot"),
};

export default defineConfig({
	output: "server",
	redirects: oldSiteRedirects,
	adapter: cloudflare(),
	image: {
		layout: "constrained",
		responsiveStyles: true,
	},
	integrations: [
		react(),
		emdash({
			database: d1({ binding: "DB", session: "auto" }),
			storage: r2({ binding: "MEDIA" }),
		}),
	],
	devToolbar: { enabled: false },
});
