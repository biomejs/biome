import { netlifyStatic } from "@astrojs/netlify";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import { h } from "hastscript";
import { escape as htmlEscape } from "html-escaper";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkToc from "remark-toc";

const anchorLinkIcon = h(
	"span",
	{ ariaHidden: "true", class: "anchor-icon" },
	h(
		"svg",
		{ width: 16, height: 16, viewBox: "0 0 24 24" },
		h("path", {
			fill: "currentcolor",
			d: "m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 0 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 1 0-1.42-1.42Zm8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 0 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 1 0 1.42 1.42l3.88-3.89a4.49 4.49 0 0 0 0-6.33ZM8.83 15.17a1 1 0 0 0 1.1.22 1 1 0 0 0 .32-.22l4.92-4.92a1 1 0 0 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42Z",
		}),
	),
);

const anchorLinkSRLabel = (text: string) =>
	h(
		"span",
		{ "is:raw": true, class: "sr-only" },
		`Section titled ${htmlEscape(text)}`,
	);

const autolinkConfig = {
	properties: { class: "anchor-link" },
	behavior: "after",
	group: ({ tagName }) =>
		h("div", { tabIndex: -1, class: `heading-wrapper level-${tagName}` }),
	content: ({ heading }) => [anchorLinkIcon, anchorLinkSRLabel("test")],
};

const site = "https://biomejs.dev";
// https://astro.build/config
export default defineConfig({
	site,
	output: "static",

	compressHTML: true,

	image: {
		domains: ["avatars.githubusercontent.com"],
	},

	integrations: [
		react(),
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
				"pt-br": {
					label: "Português",
					lang: "pt-BR",
				},
			},
			sidebar: [
				{
					label: "Blog",
					link: "../blog",
					translations: { ja: "ブログ", "zh-CN": "博客" },
				},
				{
					label: "Playground",
					link: "../playground",
					translations: {
						ja: "プレイグラウンド",
						"zh-CN": "演练场",
						"pt-BR": "Ambiente de testes",
					},
				},
				{
					label: "Guides",
					translations: { ja: "ガイド", "zh-CN": "指南", "pt-BR": "Guias" },
					items: [
						{
							label: "Getting Started",
							link: "/guides/getting-started",
							translations: {
								ja: "はじめる",
								"zh-CN": "入门",
								"pt-BR": "Primeiros passos",
							},
						},
						{
							label: "Manual installation",
							link: "/guides/manual-installation",
							translations: {
								ja: "手動インストール",
								"zh-CN": "手动安装",
								"pt-BR": "Instalação manual",
							},
						},
						{
							label: "Use Biome in big projects",
							link: "/guides/big-projects",
							translations: {
								ja: "大きなプロジェクトでのBiomeの使用方法",
								"zh-CN": "大型项目使用 Biome",
								"pt-BR": "Usando o Biome em projetos grandes",
							},
						},
						{
							label: "How Biome works",
							link: "/guides/how-biome-works",
							translations: {
								ja: "Biome の振る舞い",
								"zh-CN": "Biome 工作原理",
								"pt-BR": "Como o Biome funciona",
							},
						},
						{
							label: "Integrate Biome in your editor",
							link: "/guides/integrate-in-editor",
							translations: {
								ja: "Biome をあなたのエディタに導入する",
								"zh-CN": "编辑器中使用 Biome",
								"pt-BR": "Integrando o Biome no seu editor",
							},
						},
					],
				},
				{
					label: "Tools",
					translations: {
						ja: "ツール",
						"zh-CN": "工具",
						"pt-BR": "Ferramentas",
					},
					items: [
						{
							label: "Analyzer",
							link: "/analyzer",
							translations: {
								ja: "Analyzer",
								"zh-CN": "分析器",
								"pt-BR": "Analisador",
							},
						},
						{
							label: "Formatter",
							items: [
								{
									label: "Introduction",
									link: "/formatter",
									translations: {
										ja: "イントロダクション",
										"zh-CN": "介绍",
										"pt-BR": "Introdução",
									},
								},
								{
									label: "Differences with Prettier",
									link: "/formatter/differences-with-prettier",
									translations: {
										ja: "Prettier との違い",
										"zh-CN": "与 Prettier 的区别",
										"pt-BR": "Diferenças em relação ao Prettier",
									},
								},
								{
									label: "Formatter Option Philosophy",
									link: "/formatter/option-philosophy",
									translations: {
										ja: "Formatterオプションに対する考え方",
										"zh-CN": "格式化配置理念",
										"pt-BR": "Princípios de configuração",
									},
								},
							],
							translations: {
								"zh-CN": "格式化程序",
								"pt-BR": "Formatador",
							},
						},
						{
							label: "Linter",
							link: "/linter",
						},
						{
							label: "Lint rules",
							link: "/linter/rules",
							translations: {
								ja: "Lintルール",
								"zh-CN": "Lint 规则",
								"pt-BR": "Regras do Linter",
							},
						},
					],
				},
				{
					label: "Reference",
					translations: {
						ja: "リファレンス",
						"zh-CN": "参考",
						"pt-BR": "Referências",
					},
					items: [
						{
							label: "CLI",
							link: "/reference/cli",
						},
						{
							label: "Configuration",
							link: "/reference/configuration",
							translations: {
								ja: "設定",
								"zh-CN": "配置",
								"pt-BR": "Configuração",
							},
						},
						{
							label: "VSCode extension",
							link: "/reference/vscode",
							translations: {
								ja: "VSCode拡張機能",
								"zh-CN": "VSCode 扩展",
								"pt-BR": "Extensão do VSCode",
							},
						},
					],
				},
				{
					label: "Recipes",
					translations: { ja: "レシピ", "zh-CN": "实例", "pt-BR": "Receitas" },
					items: [
						{
							label: "Continuous Integration",
							link: "/recipes/continuous-integration",
							translations: {
								ja: "継続的インテグレーション",
								"zh-CN": "持续集成",
								"pt-BR": "Integração Contínua",
							},
						},
					],
				},
				{
					label: "Internals",
					translations: {
						ja: "内部原理",
						"zh-CN": "内部原理",
						"pt-BR": "Aspectos Internos",
					},
					items: [
						{
							label: "Philosophy",
							link: "/internals/philosophy",
							translations: {
								ja: "理念",
								"zh-CN": "理念",
								"pt-BR": "Filosofia",
							},
						},
						{
							label: "Language support",
							link: "/internals/language-support",
							translations: {
								ja: "言語サポート",
								"zh-CN": "语言支持",
								"pt-BR": "Suporte de linguagens",
							},
						},
						{
							label: "Architecture",
							link: "/internals/architecture",
							translations: {
								ja: "アーキテクチャ",
								"zh-CN": "架构",
								"pt-BR": "Arquitetura",
							},
						},
						{
							label: "Credits",
							link: "/internals/credits",
							translations: {
								ja: "クレジット",
								"zh-CN": "鸣谢",
								"pt-BR": "Créditos",
							},
						},
						{
							label: "Versioning",
							link: "/internals/versioning",
							translations: {
								ja: "バージョニング",
								"zh-CN": "版本控制",
								"pt-BR": "Versionamento",
							},
						},
						{
							label: "Changelog",
							link: "/internals/changelog",
							translations: {
								"zh-CN": "更新日志",
								"pt-BR": "Alterações",
							},
						},
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
			components: {
				SiteTitle: "./src/components/starlight/SiteTitle.astro",
				Sidebar: "./src/components/starlight/Sidebar.astro",
			},
		}),
	],

	build: {
		format: "directory",
	},

	markdown: {
		syntaxHighlight: "prism",
		remarkPlugins: [remarkToc],
		rehypePlugins: [rehypeSlug, [rehypeAutolinkHeadings, autolinkConfig]],
	},

	adapter: netlifyStatic(),

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
