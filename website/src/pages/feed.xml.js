import rss, { pagesGlobToRssItems } from "@astrojs/rss";

export async function get() {
	return rss({
		title: "Biome Blog",
		description: "",
		site: import.meta.env.SITE,
		items: await pagesGlobToRssItems(import.meta.glob("./blog/**/*.mdx")),
		customData: "<language>en-us</language>",
	});
}
