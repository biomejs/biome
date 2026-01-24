/**
 * Language configuration for mapping file extensions to Biome and Prettier settings.
 */

export interface LanguageConfig {
	/** Virtual file path for Biome (used for language detection) */
	biomeFilePath: string;
	/** Prettier parser name */
	prettierParser: string;
	/** Display name for the language */
	displayName: string;
}

/**
 * Mapping of language identifiers to their configurations.
 */
export const LANGUAGES: Record<string, LanguageConfig> = {
	js: {
		biomeFilePath: "file.js",
		prettierParser: "babel",
		displayName: "JavaScript",
	},
	javascript: {
		biomeFilePath: "file.js",
		prettierParser: "babel",
		displayName: "JavaScript",
	},
	jsx: {
		biomeFilePath: "file.jsx",
		prettierParser: "babel",
		displayName: "JSX",
	},
	ts: {
		biomeFilePath: "file.ts",
		prettierParser: "typescript",
		displayName: "TypeScript",
	},
	typescript: {
		biomeFilePath: "file.ts",
		prettierParser: "typescript",
		displayName: "TypeScript",
	},
	tsx: {
		biomeFilePath: "file.tsx",
		prettierParser: "typescript",
		displayName: "TSX",
	},
	json: {
		biomeFilePath: "file.json",
		prettierParser: "json",
		displayName: "JSON",
	},
	jsonc: {
		biomeFilePath: "file.jsonc",
		prettierParser: "json",
		displayName: "JSON with Comments",
	},
	css: {
		biomeFilePath: "file.css",
		prettierParser: "css",
		displayName: "CSS",
	},
	html: {
		biomeFilePath: "file.html",
		prettierParser: "html",
		displayName: "HTML",
	},
	graphql: {
		biomeFilePath: "file.graphql",
		prettierParser: "graphql",
		displayName: "GraphQL",
	},
	gql: {
		biomeFilePath: "file.graphql",
		prettierParser: "graphql",
		displayName: "GraphQL",
	},
	md: {
		biomeFilePath: "file.md",
		prettierParser: "markdown",
		displayName: "Markdown",
	},
	markdown: {
		biomeFilePath: "file.md",
		prettierParser: "markdown",
		displayName: "Markdown",
	},
	yaml: {
		biomeFilePath: "file.yaml",
		prettierParser: "yaml",
		displayName: "YAML",
	},
	yml: {
		biomeFilePath: "file.yaml",
		prettierParser: "yaml",
		displayName: "YAML",
	},
	svelte: {
		biomeFilePath: "file.svelte",
		prettierParser: "svelte",
		displayName: "Svelte",
	},
	astro: {
		biomeFilePath: "file.astro",
		prettierParser: "astro",
		displayName: "Astro",
	},
	vue: {
		biomeFilePath: "file.vue",
		prettierParser: "vue",
		displayName: "Vue",
	},
};

/**
 * Detect language from a file path based on its extension.
 * @param filePath - The file path to analyze
 * @returns The detected language identifier, or "js" as default
 */
export function detectLanguage(filePath?: string): string {
	if (!filePath) return "js";

	const ext = filePath.split(".").pop()?.toLowerCase();
	if (ext && ext in LANGUAGES) {
		return ext;
	}

	return "js";
}

/**
 * Get the language configuration for a given language identifier.
 * @param lang - The language identifier (e.g., "js", "ts", "json")
 * @returns The language configuration, or JavaScript config as fallback
 */
export function getLanguageConfig(lang: string): LanguageConfig {
	const normalized = lang.toLowerCase();
	return LANGUAGES[normalized] ?? LANGUAGES.js;
}

/**
 * Get a list of all supported language identifiers.
 */
export function getSupportedLanguages(): string[] {
	return Object.keys(LANGUAGES);
}
