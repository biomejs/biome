import { getCollection } from "astro:content";
import { OGImageRoute } from "astro-og-canvas";

const collectionEntries = await getCollection("docs");

/** Paths for all of our Markdown content we want to generate OG images for. */
const pages = process.env.SKIP_OG
	? []
	: Object.fromEntries(collectionEntries.map(({ slug, data }) => [slug, data]));

export const { getStaticPaths, GET } = OGImageRoute({
	param: "path",

	pages,

	getImageOptions: (_, page) => {
		return {
			title: page.title,
			description: page.description,
			logo: {
				path: "./public/img/logo-avatar.png",
				size: [200],
			},
			bgImage: {
				path: "./src/assets/bg.png",
				fit: "cover",
			},
			font: {
				title: {
					size: 72,
					lineHeight: 1.2,
					families: [
						"Obviously",
						"Inter",
						"Noto Sans",
						"Noto Sans Arabic",
						"Noto Sans SC",
						"Noto Sans TC",
						"Noto Sans JP",
						"Noto Sans KR",
					],
					weight: "Medium",
					color: [255, 255, 255],
				},
				description: {
					size: 42,
					lineHeight: 1.2,
					families: ["Inter", "Noto Sans", "Noto Sans SC", "Noto Sans JP"],
					weight: "Normal",
					color: [255, 255, 255],
				},
			},
			fonts: [
				"./src/pages/og/_fonts/inter/inter-400-normal.ttf",
				"./src/pages/og/_fonts/inter/inter-500-normal.ttf",

				"./src/pages/og/_fonts/noto-sans/noto-400-normal.ttf",
				"./src/pages/og/_fonts/noto-sans/noto-500-normal.ttf",

				"./src/pages/og/_fonts/noto-sans/chinese-simplified-400-normal.otf",
				"./src/pages/og/_fonts/noto-sans/chinese-simplified-500-normal.ttf",

				"./src/pages/og/_fonts/noto-sans/japanese-400-normal.ttf",
				"./src/pages/og/_fonts/noto-sans/japanese-500-normal.ttf",
			],
		};
	},
});
