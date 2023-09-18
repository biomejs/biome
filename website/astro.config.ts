import fs from "node:fs/promises";
import path from "node:path";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import vercel from "@astrojs/vercel/static";
import type { AstroIntegration } from "astro";
import compress from "astro-compress";
import { defineConfig } from "astro/config";
import { globby } from "globby";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkToc from "remark-toc";

function resolveFile(relative: string, parent: string, root: string): string {
	if (relative[0] === "/") {
		return `${root}${relative}`;
	} else {
		return path.resolve(path.join(parent, relative));
	}
}

const IMPORT_REGEX = /^import"(.*?)";?$/;

async function readFile(
	loc: string,
	root: string,
	cache: Files,
): Promise<string> {
	let content = cache.get(loc);
	if (content === undefined) {
		content = await fs.readFile(loc, "utf8");
		content = content.trim();
		cache.set(loc, content);
	}

	const importMatch = content.match(IMPORT_REGEX);
	if (importMatch != null) {
		return readFile(
			resolveFile(importMatch[1], path.dirname(loc), root),
			root,
			cache,
		);
	}

	return content;
}

type Files = Map<string, string>;

async function inline({
	files,
	root,
	replacements,
}: {
	files: Files;
	root: string;
	replacements: {
		regex: RegExp;
		tagBefore: string;
		tagAfter: string;
	}[];
}): Promise<void> {
	const cache: Files = new Map();

	await Promise.all(
		Array.from(files.entries(), async ([htmlPath, file]) => {
			if (htmlPath.includes("playground")) {
				return;
			}

			const matches: {
				key: string;
				match: string;
				tagBefore: string;
				tagAfter: string;
			}[] = [];

			for (const { regex, tagBefore, tagAfter } of replacements) {
				file = file.replace(regex, (match, p1) => {
					const key = `{{INLINE:${matches.length - 1}}}`;
					matches.push({ key, match: p1, tagBefore, tagAfter });
					return key;
				});
			}

			const sources: string[] = await Promise.all(
				matches.map(async ({ match }) => {
					const resolvedPath = resolveFile(match, path.dirname(htmlPath), root);
					return await readFile(resolvedPath, root, cache);
				}),
			);

			for (let i = 0; i < matches.length; i++) {
				const { key, tagBefore, tagAfter } = matches[i];
				const source = sources[i];
				const index = file.indexOf(key);
				const start = file.slice(0, index);
				const end = file.slice(index + key.length);
				file = `${start}${tagBefore}${source}${tagAfter}${end}`;
			}

			files.set(htmlPath, file);
		}),
	);
}

function inlineIntegration(): AstroIntegration {
	return {
		name: "inline",
		hooks: {
			"astro:build:done": async ({ dir }) => {
				const paths = await globby(`${dir}/**/*.html`);
				const files: Files = new Map();

				await Promise.all(
					paths.map(async (path) => {
						files.set(path, await fs.readFile(path, "utf8"));
					}),
				);

				await inline({
					files,
					root: dir.pathname,
					replacements: [
						{
							regex: /<script type="module" src="(.*?)"><\/script>/g,
							tagBefore: '<script async defer type="module">',
							tagAfter: "</script>",
						},
						{
							regex: /<link rel="stylesheet" href="(.*?)"\s*\/?>/g,
							tagBefore: "<style>",
							tagAfter: "</style>",
						},
					],
				});

				for (const [path, content] of files) {
					await fs.writeFile(path, content);
				}
			},
		},
	};
}

const site = "https://biomejs.dev";
// https://astro.build/config
export default defineConfig({
	site,
	output: "static",

	compressHTML: true,

	integrations: [
		react(),
		inlineIntegration(),
		compress({
			path: "./build",
			HTML: false,
		}),
		starlight({
			title: "",
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
							label: "Use Rome in big projects",
							link: "/guides/big-projects",
							badge: {
								text: "New",
								variant: "note",
							},
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
					label: "Internals",
					items: [
						{ label: "Philosophy", link: "/internals/philosophy" },
						{ label: "Language support", link: "/internals/language-support" },
						{ label: "Architecture", link: "/internals/architecture" },
						{ label: "Credits", link: "/internals/credits" },
						{ label: "Versioning", link: "/internals/versioning" },
						{ label: "Changelog", link: "/internals/changelog" },
					],
				},
			],
			logo: {
				light: "./src/assets/svg/biome-logo.svg",
				dark: "./src/assets/svg/biome-logo.svg",
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
				twitter: "https://twitter.com/biomejs",
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

	adapter: vercel(),

	vite: {
		plugins: [],

		worker: {
			format: "es",
		},

		server: {
			fs: {
				// https://vitejs.dev/config/server-options.html#server-fs-allow
				allow: [process.cwd(), "../packages/@biomejs/wasm-web"],
			},
		},
	},
});
