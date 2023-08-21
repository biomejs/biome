import rss from "@astrojs/rss";

export const get = () =>
	rss({
		title: "Biome Blog",
		description: "",
		site: import.meta.env.SITE,
		items: import.meta.glob("./blog/**/*.mdx"),
		customData: "<language>en-us</language>",
	});
