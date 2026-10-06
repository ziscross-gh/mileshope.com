import cloudflare from "@astrojs/cloudflare";
import react from "@astrojs/react";
import { d1 } from "@emdash-cms/cloudflare";
import { defineConfig } from "astro/config";
import emdash from "emdash/astro";

export default defineConfig({
	output: "server",
	adapter: cloudflare(),
	image: {
		layout: "constrained",
		responsiveStyles: true,
	},
	integrations: [
		react(),
		emdash({
			database: d1({ binding: "DB", session: "auto" }),
			// Image uploads need an R2 bucket. R2 is not enabled on the Cloudflare
			// account yet, so storage is left out for now. To add it: enable R2 in
			// the dashboard, add the MEDIA bucket to wrangler.jsonc, import `r2`
			// from "@emdash-cms/cloudflare" and set `storage: r2({ binding: "MEDIA" })`.
		}),
	],
	devToolbar: { enabled: false },
});
