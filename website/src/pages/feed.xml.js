import { getCollection } from "astro:content";
import rss from "@astrojs/rss";

export async function GET(context) {
	const posts = (await getCollection("blog")).sort(
		(a, b) =>
			new Date(b.data.pubDate).valueOf() - new Date(a.data.pubDate).valueOf(),
	);

	return rss({
		title: "Biome Blog",
		description: "Biome's toolchain official blog",
		site: context.site,
		items: posts.map((post) => ({
			title: post.data.title,
			pubDate: post.data.pubDate,
			description: post.data.description,
			link: `/blog/${post.slug}`,
		})),
		customData: "<language>en-us</language>",
	});
}
