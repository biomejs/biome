import { netlifyStatic } from "@astrojs/netlify";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkToc from "remark-toc";
import {searchForWorkspaceRoot} from "vite";



const site = "https://biomejs.dev";
// https://astro.build/config
export default defineConfig({
	site,
	output: "static",

	compressHTML: true,

	image: {
		domains: ["avatars.githubusercontent.com"],
	},

	devOverlay: {
		enabled: true,
	},

	integrations: [
		react({
			include: ["playground/**"]
		}),
		starlight({
			title: "Biome",
			defaultLocale: "root",
			locales: {
				root: {
					label: "English",
					lang: "en",
				},
				ja: {
					label: "日本語",
					lang: "ja",
				},
				"zh-cn": {
					label: "简体中文",
					lang: "zh-CN",
				},
			},
			sidebar: [
				{ label: "Home", link: "/" },
				{ label: "Blog", link: "/blog" },
				{
					label: "Playground",
					link: "/playground",
				},
				{
					label: "Guides",
					items: [
						{ label: "Getting Started", link: "/guides/getting-started" },
						{
							label: "Manual installation",
							link: "/guides/manual-installation",
						},
						{
							label: "Use Biome in big projects",
							link: "/guides/big-projects",
						},
						{
							label: "How Biome works",
							link: "/guides/how-biome-works",
						},
						{
							label: "Integrate Biome in your editor",
							link: "/guides/integrate-in-editor",
						},
					],
				},
				{
					label: "Tools",
					items: [
						{ label: "Analyzer", link: "/analyzer" },
						{ label: "Formatter", link: "/formatter" },
						{ label: "Linter", link: "/linter" },
						{ label: "Lint rules", link: "/linter/rules" },
					],
				},

				{
					label: "Reference",
					items: [
						{ label: "CLI", link: "/reference/cli" },
						{ label: "Configuration", link: "/reference/configuration" },
						{ label: "VSCode extension", link: "/reference/vscode" },
					],
				},
				{
					label: "Recipes",
					items: [
						{
							label: "Continuous Integration",
							link: "/recipes/continuous-integration",
						},
					],
				},
				{
					label: "Internals",
					items: [
						{ label: "Philosophy", link: "/internals/philosophy" },
						{ label: "Language support", link: "/internals/language-support" },
						{
							label: "Architecture",
							link: "/internals/architecture",
						},
						{ label: "Credits", link: "/internals/credits" },
						{ label: "Versioning", link: "/internals/versioning" },
						{ label: "Changelog", link: "/internals/changelog" },
					],
				},
			],
			logo: {
				light: "./src/assets/svg/biome-logo.svg",
				dark: "./src/assets/svg/biome-logo.svg",
				replacesTitle: true,
			},
			favicon: "/img/favicon.svg",
			head: [
				{
					tag: "link",
					attrs: {
						rel: "icon",
						href: "/images/favicon-32x32.png",
						sizes: "32x32",
					},
				},
				{
					tag: "meta",
					attrs: { property: "og:image", content: `${site}/img/og.png?v=1` },
				},
				{
					tag: "meta",
					attrs: {
						property: "twitter:image",
						content: `${site}/img/og.png?v=1`,
					},
				},
			],
			customCss: [
				// Relative path to your custom CSS file
				"./src/styles/index.scss",
			],
			social: {
				discord: "https://discord.gg/BypW39g6Yc",
				github: "https://github.com/biomejs/biome",
				"x.com": "https://twitter.com/biomejs",
				mastodon: "https://fosstodon.org/@biomejs",
			},
			editLink: {
				baseUrl: "https://github.com/biomejs/biome/edit/main/website/",
			},
		}),
	],

	build: {
		format: "directory",
	},

	markdown: {
		syntaxHighlight: "prism",
		remarkPlugins: [remarkToc],
		rehypePlugins: [
			rehypeSlug,
			[
				rehypeAutolinkHeadings,
				{
					behavior: "append",
					content: [],
				},
			],
		],
	},

	adapter: netlifyStatic(),

	vite: {
		plugins: [],

		worker: {
			format: "es",
		},

		server: {
			fs: {
		// 		https://vitejs.dev/config/server-options.html#server-fs-allow
				allow: [searchForWorkspaceRoot(process.cwd()), "../packages/@biomejs/wasm-web"],
			},
		},
	},
});
