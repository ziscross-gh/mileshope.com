/** Small helpers shared by the pages. */

/** Sessions in the order set by their "Order on the page" field. */
export function sortSessions<T extends { data: { sort_order?: number | null } }>(sessions: T[]): T[] {
	return [...sessions].sort((a, b) => (a.data.sort_order ?? 99) - (b.data.sort_order ?? 99));
}

/** The twelve signs in wheel order, for the zodiac strip. */
export const SIGNS = [
	["aries", "Aries"],
	["taurus", "Taurus"],
	["gemini", "Gemini"],
	["cancer", "Cancer"],
	["leo", "Leo"],
	["virgo", "Virgo"],
	["libra", "Libra"],
	["scorpio", "Scorpio"],
	["sagittarius", "Sagittarius"],
	["capricorn", "Capricorn"],
	["aquarius", "Aquarius"],
	["pisces", "Pisces"],
] as const;

/**
 * The public contact address. Cloudflare Email Routing forwards it to Hope's
 * inbox, so changing it here also needs a new routing rule on Cloudflare.
 */
export const CONTACT_EMAIL = "hello@mileshope.com";
