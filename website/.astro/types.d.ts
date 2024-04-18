declare module 'astro:content' {
	interface Render {
		'.mdx': Promise<{
			Content: import('astro').MarkdownInstance<{}>['Content'];
			headings: import('astro').MarkdownHeading[];
			remarkPluginFrontmatter: Record<string, any>;
		}>;
	}
}

declare module 'astro:content' {
	interface Render {
		'.md': Promise<{
			Content: import('astro').MarkdownInstance<{}>['Content'];
			headings: import('astro').MarkdownHeading[];
			remarkPluginFrontmatter: Record<string, any>;
		}>;
	}
}

declare module 'astro:content' {
	type Flatten<T> = T extends { [K: string]: infer U } ? U : never;

	export type CollectionKey = keyof AnyEntryMap;
	export type CollectionEntry<C extends CollectionKey> = Flatten<AnyEntryMap[C]>;

	export type ContentCollectionKey = keyof ContentEntryMap;
	export type DataCollectionKey = keyof DataEntryMap;

	type AllValuesOf<T> = T extends any ? T[keyof T] : never;
	type ValidContentEntrySlug<C extends keyof ContentEntryMap> = AllValuesOf<
		ContentEntryMap[C]
	>['slug'];

	export function getEntryBySlug<
		C extends keyof ContentEntryMap,
		E extends ValidContentEntrySlug<C> | (string & {}),
	>(
		collection: C,
		// Note that this has to accept a regular string too, for SSR
		entrySlug: E
	): E extends ValidContentEntrySlug<C>
		? Promise<CollectionEntry<C>>
		: Promise<CollectionEntry<C> | undefined>;

	export function getDataEntryById<C extends keyof DataEntryMap, E extends keyof DataEntryMap[C]>(
		collection: C,
		entryId: E
	): Promise<CollectionEntry<C>>;

	export function getCollection<C extends keyof AnyEntryMap, E extends CollectionEntry<C>>(
		collection: C,
		filter?: (entry: CollectionEntry<C>) => entry is E
	): Promise<E[]>;
	export function getCollection<C extends keyof AnyEntryMap>(
		collection: C,
		filter?: (entry: CollectionEntry<C>) => unknown
	): Promise<CollectionEntry<C>[]>;

	export function getEntry<
		C extends keyof ContentEntryMap,
		E extends ValidContentEntrySlug<C> | (string & {}),
	>(entry: {
		collection: C;
		slug: E;
	}): E extends ValidContentEntrySlug<C>
		? Promise<CollectionEntry<C>>
		: Promise<CollectionEntry<C> | undefined>;
	export function getEntry<
		C extends keyof DataEntryMap,
		E extends keyof DataEntryMap[C] | (string & {}),
	>(entry: {
		collection: C;
		id: E;
	}): E extends keyof DataEntryMap[C]
		? Promise<DataEntryMap[C][E]>
		: Promise<CollectionEntry<C> | undefined>;
	export function getEntry<
		C extends keyof ContentEntryMap,
		E extends ValidContentEntrySlug<C> | (string & {}),
	>(
		collection: C,
		slug: E
	): E extends ValidContentEntrySlug<C>
		? Promise<CollectionEntry<C>>
		: Promise<CollectionEntry<C> | undefined>;
	export function getEntry<
		C extends keyof DataEntryMap,
		E extends keyof DataEntryMap[C] | (string & {}),
	>(
		collection: C,
		id: E
	): E extends keyof DataEntryMap[C]
		? Promise<DataEntryMap[C][E]>
		: Promise<CollectionEntry<C> | undefined>;

	/** Resolve an array of entry references from the same collection */
	export function getEntries<C extends keyof ContentEntryMap>(
		entries: {
			collection: C;
			slug: ValidContentEntrySlug<C>;
		}[]
	): Promise<CollectionEntry<C>[]>;
	export function getEntries<C extends keyof DataEntryMap>(
		entries: {
			collection: C;
			id: keyof DataEntryMap[C];
		}[]
	): Promise<CollectionEntry<C>[]>;

	export function reference<C extends keyof AnyEntryMap>(
		collection: C
	): import('astro/zod').ZodEffects<
		import('astro/zod').ZodString,
		C extends keyof ContentEntryMap
			? {
					collection: C;
					slug: ValidContentEntrySlug<C>;
				}
			: {
					collection: C;
					id: keyof DataEntryMap[C];
				}
	>;
	// Allow generic `string` to avoid excessive type errors in the config
	// if `dev` is not running to update as you edit.
	// Invalid collection names will be caught at build time.
	export function reference<C extends string>(
		collection: C
	): import('astro/zod').ZodEffects<import('astro/zod').ZodString, never>;

	type ReturnTypeOrOriginal<T> = T extends (...args: any[]) => infer R ? R : T;
	type InferEntrySchema<C extends keyof AnyEntryMap> = import('astro/zod').infer<
		ReturnTypeOrOriginal<Required<ContentConfig['collections'][C]>['schema']>
	>;

	type ContentEntryMap = {
		"blog": {
"annoucing-biome.mdx": {
	id: "annoucing-biome.mdx";
  slug: "annoucing-biome";
  body: string;
  collection: "blog";
  data: InferEntrySchema<"blog">
} & { render(): Render[".mdx"] };
"biome-v1-5.md": {
	id: "biome-v1-5.md";
  slug: "biome-v1-5";
  body: string;
  collection: "blog";
  data: InferEntrySchema<"blog">
} & { render(): Render[".md"] };
"biome-v1.mdx": {
	id: "biome-v1.mdx";
  slug: "biome-v1";
  body: string;
  collection: "blog";
  data: InferEntrySchema<"blog">
} & { render(): Render[".mdx"] };
"biome-wins-prettier-challenge.md": {
	id: "biome-wins-prettier-challenge.md";
  slug: "biome-wins-prettier-challenge";
  body: string;
  collection: "blog";
  data: InferEntrySchema<"blog">
} & { render(): Render[".md"] };
"roadmap-2024.md": {
	id: "roadmap-2024.md";
  slug: "roadmap-2024";
  body: string;
  collection: "blog";
  data: InferEntrySchema<"blog">
} & { render(): Render[".md"] };
};
"docs": {
"404.md": {
	id: "404.md";
  slug: "404";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"analyzer/index.mdx": {
	id: "analyzer/index.mdx";
  slug: "analyzer";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"formatter/differences-with-prettier.md": {
	id: "formatter/differences-with-prettier.md";
  slug: "formatter/differences-with-prettier";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"formatter/index.mdx": {
	id: "formatter/index.mdx";
  slug: "formatter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"formatter/option-philosophy.md": {
	id: "formatter/option-philosophy.md";
  slug: "formatter/option-philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"guides/big-projects.mdx": {
	id: "guides/big-projects.mdx";
  slug: "guides/big-projects";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"guides/getting-started.mdx": {
	id: "guides/getting-started.mdx";
  slug: "guides/getting-started";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"guides/how-biome-works.mdx": {
	id: "guides/how-biome-works.mdx";
  slug: "guides/how-biome-works";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"guides/integrate-in-editor.mdx": {
	id: "guides/integrate-in-editor.mdx";
  slug: "guides/integrate-in-editor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"guides/integrate-in-vcs.mdx": {
	id: "guides/integrate-in-vcs.mdx";
  slug: "guides/integrate-in-vcs";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"guides/manual-installation.mdx": {
	id: "guides/manual-installation.mdx";
  slug: "guides/manual-installation";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"index.mdx": {
	id: "index.mdx";
  slug: "index";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"internals/architecture.mdx": {
	id: "internals/architecture.mdx";
  slug: "internals/architecture";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"internals/changelog.md": {
	id: "internals/changelog.md";
  slug: "internals/changelog";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"internals/credits.mdx": {
	id: "internals/credits.mdx";
  slug: "internals/credits";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"internals/language-support.mdx": {
	id: "internals/language-support.mdx";
  slug: "internals/language-support";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"internals/philosophy.mdx": {
	id: "internals/philosophy.mdx";
  slug: "internals/philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"internals/versioning.mdx": {
	id: "internals/versioning.mdx";
  slug: "internals/versioning";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/404.md": {
	id: "ja/404.md";
  slug: "ja/404";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"ja/analyzer/index.mdx": {
	id: "ja/analyzer/index.mdx";
  slug: "ja/analyzer";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/formatter/differences-with-prettier.md": {
	id: "ja/formatter/differences-with-prettier.md";
  slug: "ja/formatter/differences-with-prettier";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"ja/formatter/index.mdx": {
	id: "ja/formatter/index.mdx";
  slug: "ja/formatter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/formatter/option-philosophy.md": {
	id: "ja/formatter/option-philosophy.md";
  slug: "ja/formatter/option-philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"ja/guides/big-projects.mdx": {
	id: "ja/guides/big-projects.mdx";
  slug: "ja/guides/big-projects";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/guides/getting-started.mdx": {
	id: "ja/guides/getting-started.mdx";
  slug: "ja/guides/getting-started";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/guides/how-biome-works.mdx": {
	id: "ja/guides/how-biome-works.mdx";
  slug: "ja/guides/how-biome-works";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/guides/integrate-in-editor.mdx": {
	id: "ja/guides/integrate-in-editor.mdx";
  slug: "ja/guides/integrate-in-editor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/guides/manual-installation.mdx": {
	id: "ja/guides/manual-installation.mdx";
  slug: "ja/guides/manual-installation";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/index.mdx": {
	id: "ja/index.mdx";
  slug: "ja";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/internals/architecture.mdx": {
	id: "ja/internals/architecture.mdx";
  slug: "ja/internals/architecture";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/internals/credits.mdx": {
	id: "ja/internals/credits.mdx";
  slug: "ja/internals/credits";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/internals/language-support.mdx": {
	id: "ja/internals/language-support.mdx";
  slug: "ja/internals/language-support";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/internals/philosophy.mdx": {
	id: "ja/internals/philosophy.mdx";
  slug: "ja/internals/philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/internals/versioning.mdx": {
	id: "ja/internals/versioning.mdx";
  slug: "ja/internals/versioning";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/linter/index.mdx": {
	id: "ja/linter/index.mdx";
  slug: "ja/linter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/recipes/continuous-integration.mdx": {
	id: "ja/recipes/continuous-integration.mdx";
  slug: "ja/recipes/continuous-integration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/recipes/git-hooks.mdx": {
	id: "ja/recipes/git-hooks.mdx";
  slug: "ja/recipes/git-hooks";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"ja/reference/vscode.mdx": {
	id: "ja/reference/vscode.mdx";
  slug: "ja/reference/vscode";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"linter/index.mdx": {
	id: "linter/index.mdx";
  slug: "linter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"linter/rules-sources.mdx": {
	id: "linter/rules-sources.mdx";
  slug: "linter/rules-sources";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"linter/rules/index.mdx": {
	id: "linter/rules/index.mdx";
  slug: "linter/rules";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"linter/rules/no-access-key.md": {
	id: "linter/rules/no-access-key.md";
  slug: "linter/rules/no-access-key";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-accumulating-spread.md": {
	id: "linter/rules/no-accumulating-spread.md";
  slug: "linter/rules/no-accumulating-spread";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-approximative-numeric-constant.md": {
	id: "linter/rules/no-approximative-numeric-constant.md";
  slug: "linter/rules/no-approximative-numeric-constant";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-arguments.md": {
	id: "linter/rules/no-arguments.md";
  slug: "linter/rules/no-arguments";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-aria-hidden-on-focusable.md": {
	id: "linter/rules/no-aria-hidden-on-focusable.md";
  slug: "linter/rules/no-aria-hidden-on-focusable";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-aria-unsupported-elements.md": {
	id: "linter/rules/no-aria-unsupported-elements.md";
  slug: "linter/rules/no-aria-unsupported-elements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-array-index-key.md": {
	id: "linter/rules/no-array-index-key.md";
  slug: "linter/rules/no-array-index-key";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-assign-in-expressions.md": {
	id: "linter/rules/no-assign-in-expressions.md";
  slug: "linter/rules/no-assign-in-expressions";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-async-promise-executor.md": {
	id: "linter/rules/no-async-promise-executor.md";
  slug: "linter/rules/no-async-promise-executor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-autofocus.md": {
	id: "linter/rules/no-autofocus.md";
  slug: "linter/rules/no-autofocus";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-banned-types.md": {
	id: "linter/rules/no-banned-types.md";
  slug: "linter/rules/no-banned-types";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-barrel-file.md": {
	id: "linter/rules/no-barrel-file.md";
  slug: "linter/rules/no-barrel-file";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-blank-target.md": {
	id: "linter/rules/no-blank-target.md";
  slug: "linter/rules/no-blank-target";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-catch-assign.md": {
	id: "linter/rules/no-catch-assign.md";
  slug: "linter/rules/no-catch-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-children-prop.md": {
	id: "linter/rules/no-children-prop.md";
  slug: "linter/rules/no-children-prop";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-class-assign.md": {
	id: "linter/rules/no-class-assign.md";
  slug: "linter/rules/no-class-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-comma-operator.md": {
	id: "linter/rules/no-comma-operator.md";
  slug: "linter/rules/no-comma-operator";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-comment-text.md": {
	id: "linter/rules/no-comment-text.md";
  slug: "linter/rules/no-comment-text";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-compare-neg-zero.md": {
	id: "linter/rules/no-compare-neg-zero.md";
  slug: "linter/rules/no-compare-neg-zero";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-confusing-labels.md": {
	id: "linter/rules/no-confusing-labels.md";
  slug: "linter/rules/no-confusing-labels";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-confusing-void-type.md": {
	id: "linter/rules/no-confusing-void-type.md";
  slug: "linter/rules/no-confusing-void-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-console-log.md": {
	id: "linter/rules/no-console-log.md";
  slug: "linter/rules/no-console-log";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-console.md": {
	id: "linter/rules/no-console.md";
  slug: "linter/rules/no-console";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-const-assign.md": {
	id: "linter/rules/no-const-assign.md";
  slug: "linter/rules/no-const-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-const-enum.md": {
	id: "linter/rules/no-const-enum.md";
  slug: "linter/rules/no-const-enum";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-constant-condition.md": {
	id: "linter/rules/no-constant-condition.md";
  slug: "linter/rules/no-constant-condition";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-constructor-return.md": {
	id: "linter/rules/no-constructor-return.md";
  slug: "linter/rules/no-constructor-return";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-control-characters-in-regex.md": {
	id: "linter/rules/no-control-characters-in-regex.md";
  slug: "linter/rules/no-control-characters-in-regex";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-dangerously-set-inner-html-with-children.md": {
	id: "linter/rules/no-dangerously-set-inner-html-with-children.md";
  slug: "linter/rules/no-dangerously-set-inner-html-with-children";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-dangerously-set-inner-html.md": {
	id: "linter/rules/no-dangerously-set-inner-html.md";
  slug: "linter/rules/no-dangerously-set-inner-html";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-debugger.md": {
	id: "linter/rules/no-debugger.md";
  slug: "linter/rules/no-debugger";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-default-export.md": {
	id: "linter/rules/no-default-export.md";
  slug: "linter/rules/no-default-export";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-delete.md": {
	id: "linter/rules/no-delete.md";
  slug: "linter/rules/no-delete";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-distracting-elements.md": {
	id: "linter/rules/no-distracting-elements.md";
  slug: "linter/rules/no-distracting-elements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-double-equals.md": {
	id: "linter/rules/no-double-equals.md";
  slug: "linter/rules/no-double-equals";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-case.md": {
	id: "linter/rules/no-duplicate-case.md";
  slug: "linter/rules/no-duplicate-case";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-class-members.md": {
	id: "linter/rules/no-duplicate-class-members.md";
  slug: "linter/rules/no-duplicate-class-members";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-json-keys.md": {
	id: "linter/rules/no-duplicate-json-keys.md";
  slug: "linter/rules/no-duplicate-json-keys";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-jsx-props.md": {
	id: "linter/rules/no-duplicate-jsx-props.md";
  slug: "linter/rules/no-duplicate-jsx-props";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-object-keys.md": {
	id: "linter/rules/no-duplicate-object-keys.md";
  slug: "linter/rules/no-duplicate-object-keys";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-parameters.md": {
	id: "linter/rules/no-duplicate-parameters.md";
  slug: "linter/rules/no-duplicate-parameters";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-duplicate-test-hooks.md": {
	id: "linter/rules/no-duplicate-test-hooks.md";
  slug: "linter/rules/no-duplicate-test-hooks";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-empty-block-statements.md": {
	id: "linter/rules/no-empty-block-statements.md";
  slug: "linter/rules/no-empty-block-statements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-empty-character-class-in-regex.md": {
	id: "linter/rules/no-empty-character-class-in-regex.md";
  slug: "linter/rules/no-empty-character-class-in-regex";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-empty-interface.md": {
	id: "linter/rules/no-empty-interface.md";
  slug: "linter/rules/no-empty-interface";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-empty-pattern.md": {
	id: "linter/rules/no-empty-pattern.md";
  slug: "linter/rules/no-empty-pattern";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-empty-type-parameters.md": {
	id: "linter/rules/no-empty-type-parameters.md";
  slug: "linter/rules/no-empty-type-parameters";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-excessive-cognitive-complexity.md": {
	id: "linter/rules/no-excessive-cognitive-complexity.md";
  slug: "linter/rules/no-excessive-cognitive-complexity";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-excessive-nested-test-suites.md": {
	id: "linter/rules/no-excessive-nested-test-suites.md";
  slug: "linter/rules/no-excessive-nested-test-suites";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-explicit-any.md": {
	id: "linter/rules/no-explicit-any.md";
  slug: "linter/rules/no-explicit-any";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-exports-in-test.md": {
	id: "linter/rules/no-exports-in-test.md";
  slug: "linter/rules/no-exports-in-test";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-extra-boolean-cast.md": {
	id: "linter/rules/no-extra-boolean-cast.md";
  slug: "linter/rules/no-extra-boolean-cast";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-extra-non-null-assertion.md": {
	id: "linter/rules/no-extra-non-null-assertion.md";
  slug: "linter/rules/no-extra-non-null-assertion";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-fallthrough-switch-clause.md": {
	id: "linter/rules/no-fallthrough-switch-clause.md";
  slug: "linter/rules/no-fallthrough-switch-clause";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-focused-tests.md": {
	id: "linter/rules/no-focused-tests.md";
  slug: "linter/rules/no-focused-tests";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-for-each.md": {
	id: "linter/rules/no-for-each.md";
  slug: "linter/rules/no-for-each";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-function-assign.md": {
	id: "linter/rules/no-function-assign.md";
  slug: "linter/rules/no-function-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-global-assign.md": {
	id: "linter/rules/no-global-assign.md";
  slug: "linter/rules/no-global-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-global-eval.md": {
	id: "linter/rules/no-global-eval.md";
  slug: "linter/rules/no-global-eval";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-global-is-finite.md": {
	id: "linter/rules/no-global-is-finite.md";
  slug: "linter/rules/no-global-is-finite";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-global-is-nan.md": {
	id: "linter/rules/no-global-is-nan.md";
  slug: "linter/rules/no-global-is-nan";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-global-object-calls.md": {
	id: "linter/rules/no-global-object-calls.md";
  slug: "linter/rules/no-global-object-calls";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-header-scope.md": {
	id: "linter/rules/no-header-scope.md";
  slug: "linter/rules/no-header-scope";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-implicit-any-let.md": {
	id: "linter/rules/no-implicit-any-let.md";
  slug: "linter/rules/no-implicit-any-let";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-implicit-boolean.md": {
	id: "linter/rules/no-implicit-boolean.md";
  slug: "linter/rules/no-implicit-boolean";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-import-assign.md": {
	id: "linter/rules/no-import-assign.md";
  slug: "linter/rules/no-import-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-inferrable-types.md": {
	id: "linter/rules/no-inferrable-types.md";
  slug: "linter/rules/no-inferrable-types";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-inner-declarations.md": {
	id: "linter/rules/no-inner-declarations.md";
  slug: "linter/rules/no-inner-declarations";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-interactive-element-to-noninteractive-role.md": {
	id: "linter/rules/no-interactive-element-to-noninteractive-role.md";
  slug: "linter/rules/no-interactive-element-to-noninteractive-role";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-invalid-constructor-super.md": {
	id: "linter/rules/no-invalid-constructor-super.md";
  slug: "linter/rules/no-invalid-constructor-super";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-invalid-new-builtin.md": {
	id: "linter/rules/no-invalid-new-builtin.md";
  slug: "linter/rules/no-invalid-new-builtin";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-invalid-use-before-declaration.md": {
	id: "linter/rules/no-invalid-use-before-declaration.md";
  slug: "linter/rules/no-invalid-use-before-declaration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-label-var.md": {
	id: "linter/rules/no-label-var.md";
  slug: "linter/rules/no-label-var";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-misleading-character-class.md": {
	id: "linter/rules/no-misleading-character-class.md";
  slug: "linter/rules/no-misleading-character-class";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-misleading-instantiator.md": {
	id: "linter/rules/no-misleading-instantiator.md";
  slug: "linter/rules/no-misleading-instantiator";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-misrefactored-shorthand-assign.md": {
	id: "linter/rules/no-misrefactored-shorthand-assign.md";
  slug: "linter/rules/no-misrefactored-shorthand-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-multiple-spaces-in-regular-expression-literals.md": {
	id: "linter/rules/no-multiple-spaces-in-regular-expression-literals.md";
  slug: "linter/rules/no-multiple-spaces-in-regular-expression-literals";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-namespace-import.md": {
	id: "linter/rules/no-namespace-import.md";
  slug: "linter/rules/no-namespace-import";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-namespace.md": {
	id: "linter/rules/no-namespace.md";
  slug: "linter/rules/no-namespace";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-negation-else.md": {
	id: "linter/rules/no-negation-else.md";
  slug: "linter/rules/no-negation-else";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-new-symbol.md": {
	id: "linter/rules/no-new-symbol.md";
  slug: "linter/rules/no-new-symbol";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-nodejs-modules.md": {
	id: "linter/rules/no-nodejs-modules.md";
  slug: "linter/rules/no-nodejs-modules";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-non-null-assertion.md": {
	id: "linter/rules/no-non-null-assertion.md";
  slug: "linter/rules/no-non-null-assertion";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-noninteractive-element-to-interactive-role.md": {
	id: "linter/rules/no-noninteractive-element-to-interactive-role.md";
  slug: "linter/rules/no-noninteractive-element-to-interactive-role";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-noninteractive-tabindex.md": {
	id: "linter/rules/no-noninteractive-tabindex.md";
  slug: "linter/rules/no-noninteractive-tabindex";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-nonoctal-decimal-escape.md": {
	id: "linter/rules/no-nonoctal-decimal-escape.md";
  slug: "linter/rules/no-nonoctal-decimal-escape";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-parameter-assign.md": {
	id: "linter/rules/no-parameter-assign.md";
  slug: "linter/rules/no-parameter-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-parameter-properties.md": {
	id: "linter/rules/no-parameter-properties.md";
  slug: "linter/rules/no-parameter-properties";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-positive-tabindex.md": {
	id: "linter/rules/no-positive-tabindex.md";
  slug: "linter/rules/no-positive-tabindex";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-precision-loss.md": {
	id: "linter/rules/no-precision-loss.md";
  slug: "linter/rules/no-precision-loss";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-prototype-builtins.md": {
	id: "linter/rules/no-prototype-builtins.md";
  slug: "linter/rules/no-prototype-builtins";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-re-export-all.md": {
	id: "linter/rules/no-re-export-all.md";
  slug: "linter/rules/no-re-export-all";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-redeclare.md": {
	id: "linter/rules/no-redeclare.md";
  slug: "linter/rules/no-redeclare";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-redundant-alt.md": {
	id: "linter/rules/no-redundant-alt.md";
  slug: "linter/rules/no-redundant-alt";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-redundant-roles.md": {
	id: "linter/rules/no-redundant-roles.md";
  slug: "linter/rules/no-redundant-roles";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-redundant-use-strict.md": {
	id: "linter/rules/no-redundant-use-strict.md";
  slug: "linter/rules/no-redundant-use-strict";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-render-return-value.md": {
	id: "linter/rules/no-render-return-value.md";
  slug: "linter/rules/no-render-return-value";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-restricted-globals.md": {
	id: "linter/rules/no-restricted-globals.md";
  slug: "linter/rules/no-restricted-globals";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-restricted-imports.md": {
	id: "linter/rules/no-restricted-imports.md";
  slug: "linter/rules/no-restricted-imports";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-self-assign.md": {
	id: "linter/rules/no-self-assign.md";
  slug: "linter/rules/no-self-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-self-compare.md": {
	id: "linter/rules/no-self-compare.md";
  slug: "linter/rules/no-self-compare";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-semicolon-in-jsx.md": {
	id: "linter/rules/no-semicolon-in-jsx.md";
  slug: "linter/rules/no-semicolon-in-jsx";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-setter-return.md": {
	id: "linter/rules/no-setter-return.md";
  slug: "linter/rules/no-setter-return";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-shadow-restricted-names.md": {
	id: "linter/rules/no-shadow-restricted-names.md";
  slug: "linter/rules/no-shadow-restricted-names";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-shouty-constants.md": {
	id: "linter/rules/no-shouty-constants.md";
  slug: "linter/rules/no-shouty-constants";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-skipped-tests.md": {
	id: "linter/rules/no-skipped-tests.md";
  slug: "linter/rules/no-skipped-tests";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-sparse-array.md": {
	id: "linter/rules/no-sparse-array.md";
  slug: "linter/rules/no-sparse-array";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-static-only-class.md": {
	id: "linter/rules/no-static-only-class.md";
  slug: "linter/rules/no-static-only-class";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-string-case-mismatch.md": {
	id: "linter/rules/no-string-case-mismatch.md";
  slug: "linter/rules/no-string-case-mismatch";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-svg-without-title.md": {
	id: "linter/rules/no-svg-without-title.md";
  slug: "linter/rules/no-svg-without-title";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-switch-declarations.md": {
	id: "linter/rules/no-switch-declarations.md";
  slug: "linter/rules/no-switch-declarations";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-then-property.md": {
	id: "linter/rules/no-then-property.md";
  slug: "linter/rules/no-then-property";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-this-in-static.md": {
	id: "linter/rules/no-this-in-static.md";
  slug: "linter/rules/no-this-in-static";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-undeclared-dependencies.md": {
	id: "linter/rules/no-undeclared-dependencies.md";
  slug: "linter/rules/no-undeclared-dependencies";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-undeclared-variables.md": {
	id: "linter/rules/no-undeclared-variables.md";
  slug: "linter/rules/no-undeclared-variables";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unnecessary-continue.md": {
	id: "linter/rules/no-unnecessary-continue.md";
  slug: "linter/rules/no-unnecessary-continue";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unreachable-super.md": {
	id: "linter/rules/no-unreachable-super.md";
  slug: "linter/rules/no-unreachable-super";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unreachable.md": {
	id: "linter/rules/no-unreachable.md";
  slug: "linter/rules/no-unreachable";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unsafe-declaration-merging.md": {
	id: "linter/rules/no-unsafe-declaration-merging.md";
  slug: "linter/rules/no-unsafe-declaration-merging";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unsafe-finally.md": {
	id: "linter/rules/no-unsafe-finally.md";
  slug: "linter/rules/no-unsafe-finally";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unsafe-negation.md": {
	id: "linter/rules/no-unsafe-negation.md";
  slug: "linter/rules/no-unsafe-negation";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unsafe-optional-chaining.md": {
	id: "linter/rules/no-unsafe-optional-chaining.md";
  slug: "linter/rules/no-unsafe-optional-chaining";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unused-imports.md": {
	id: "linter/rules/no-unused-imports.md";
  slug: "linter/rules/no-unused-imports";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unused-labels.md": {
	id: "linter/rules/no-unused-labels.md";
  slug: "linter/rules/no-unused-labels";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unused-private-class-members.md": {
	id: "linter/rules/no-unused-private-class-members.md";
  slug: "linter/rules/no-unused-private-class-members";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unused-template-literal.md": {
	id: "linter/rules/no-unused-template-literal.md";
  slug: "linter/rules/no-unused-template-literal";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-unused-variables.md": {
	id: "linter/rules/no-unused-variables.md";
  slug: "linter/rules/no-unused-variables";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-catch.md": {
	id: "linter/rules/no-useless-catch.md";
  slug: "linter/rules/no-useless-catch";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-constructor.md": {
	id: "linter/rules/no-useless-constructor.md";
  slug: "linter/rules/no-useless-constructor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-else.md": {
	id: "linter/rules/no-useless-else.md";
  slug: "linter/rules/no-useless-else";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-empty-export.md": {
	id: "linter/rules/no-useless-empty-export.md";
  slug: "linter/rules/no-useless-empty-export";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-fragments.md": {
	id: "linter/rules/no-useless-fragments.md";
  slug: "linter/rules/no-useless-fragments";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-label.md": {
	id: "linter/rules/no-useless-label.md";
  slug: "linter/rules/no-useless-label";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-lone-block-statements.md": {
	id: "linter/rules/no-useless-lone-block-statements.md";
  slug: "linter/rules/no-useless-lone-block-statements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-rename.md": {
	id: "linter/rules/no-useless-rename.md";
  slug: "linter/rules/no-useless-rename";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-switch-case.md": {
	id: "linter/rules/no-useless-switch-case.md";
  slug: "linter/rules/no-useless-switch-case";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-ternary.md": {
	id: "linter/rules/no-useless-ternary.md";
  slug: "linter/rules/no-useless-ternary";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-this-alias.md": {
	id: "linter/rules/no-useless-this-alias.md";
  slug: "linter/rules/no-useless-this-alias";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-useless-type-constraint.md": {
	id: "linter/rules/no-useless-type-constraint.md";
  slug: "linter/rules/no-useless-type-constraint";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-var.md": {
	id: "linter/rules/no-var.md";
  slug: "linter/rules/no-var";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-void-elements-with-children.md": {
	id: "linter/rules/no-void-elements-with-children.md";
  slug: "linter/rules/no-void-elements-with-children";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-void-type-return.md": {
	id: "linter/rules/no-void-type-return.md";
  slug: "linter/rules/no-void-type-return";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-void.md": {
	id: "linter/rules/no-void.md";
  slug: "linter/rules/no-void";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/no-with.md": {
	id: "linter/rules/no-with.md";
  slug: "linter/rules/no-with";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-alt-text.md": {
	id: "linter/rules/use-alt-text.md";
  slug: "linter/rules/use-alt-text";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-anchor-content.md": {
	id: "linter/rules/use-anchor-content.md";
  slug: "linter/rules/use-anchor-content";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-aria-activedescendant-with-tabindex.md": {
	id: "linter/rules/use-aria-activedescendant-with-tabindex.md";
  slug: "linter/rules/use-aria-activedescendant-with-tabindex";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-aria-props-for-role.md": {
	id: "linter/rules/use-aria-props-for-role.md";
  slug: "linter/rules/use-aria-props-for-role";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-arrow-function.md": {
	id: "linter/rules/use-arrow-function.md";
  slug: "linter/rules/use-arrow-function";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-as-const-assertion.md": {
	id: "linter/rules/use-as-const-assertion.md";
  slug: "linter/rules/use-as-const-assertion";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-await.md": {
	id: "linter/rules/use-await.md";
  slug: "linter/rules/use-await";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-block-statements.md": {
	id: "linter/rules/use-block-statements.md";
  slug: "linter/rules/use-block-statements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-button-type.md": {
	id: "linter/rules/use-button-type.md";
  slug: "linter/rules/use-button-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-collapsed-else-if.md": {
	id: "linter/rules/use-collapsed-else-if.md";
  slug: "linter/rules/use-collapsed-else-if";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-consistent-array-type.md": {
	id: "linter/rules/use-consistent-array-type.md";
  slug: "linter/rules/use-consistent-array-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-const.md": {
	id: "linter/rules/use-const.md";
  slug: "linter/rules/use-const";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-default-parameter-last.md": {
	id: "linter/rules/use-default-parameter-last.md";
  slug: "linter/rules/use-default-parameter-last";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-default-switch-clause-last.md": {
	id: "linter/rules/use-default-switch-clause-last.md";
  slug: "linter/rules/use-default-switch-clause-last";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-enum-initializers.md": {
	id: "linter/rules/use-enum-initializers.md";
  slug: "linter/rules/use-enum-initializers";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-exhaustive-dependencies.md": {
	id: "linter/rules/use-exhaustive-dependencies.md";
  slug: "linter/rules/use-exhaustive-dependencies";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-exponentiation-operator.md": {
	id: "linter/rules/use-exponentiation-operator.md";
  slug: "linter/rules/use-exponentiation-operator";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-export-type.md": {
	id: "linter/rules/use-export-type.md";
  slug: "linter/rules/use-export-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-filenaming-convention.md": {
	id: "linter/rules/use-filenaming-convention.md";
  slug: "linter/rules/use-filenaming-convention";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-flat-map.md": {
	id: "linter/rules/use-flat-map.md";
  slug: "linter/rules/use-flat-map";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-for-of.md": {
	id: "linter/rules/use-for-of.md";
  slug: "linter/rules/use-for-of";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-fragment-syntax.md": {
	id: "linter/rules/use-fragment-syntax.md";
  slug: "linter/rules/use-fragment-syntax";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-getter-return.md": {
	id: "linter/rules/use-getter-return.md";
  slug: "linter/rules/use-getter-return";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-grouped-type-import.md": {
	id: "linter/rules/use-grouped-type-import.md";
  slug: "linter/rules/use-grouped-type-import";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-heading-content.md": {
	id: "linter/rules/use-heading-content.md";
  slug: "linter/rules/use-heading-content";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-hook-at-top-level.md": {
	id: "linter/rules/use-hook-at-top-level.md";
  slug: "linter/rules/use-hook-at-top-level";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-html-lang.md": {
	id: "linter/rules/use-html-lang.md";
  slug: "linter/rules/use-html-lang";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-iframe-title.md": {
	id: "linter/rules/use-iframe-title.md";
  slug: "linter/rules/use-iframe-title";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-import-restrictions.md": {
	id: "linter/rules/use-import-restrictions.md";
  slug: "linter/rules/use-import-restrictions";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-import-type.md": {
	id: "linter/rules/use-import-type.md";
  slug: "linter/rules/use-import-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-is-array.md": {
	id: "linter/rules/use-is-array.md";
  slug: "linter/rules/use-is-array";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-is-nan.md": {
	id: "linter/rules/use-is-nan.md";
  slug: "linter/rules/use-is-nan";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-jsx-key-in-iterable.md": {
	id: "linter/rules/use-jsx-key-in-iterable.md";
  slug: "linter/rules/use-jsx-key-in-iterable";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-key-with-click-events.md": {
	id: "linter/rules/use-key-with-click-events.md";
  slug: "linter/rules/use-key-with-click-events";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-key-with-mouse-events.md": {
	id: "linter/rules/use-key-with-mouse-events.md";
  slug: "linter/rules/use-key-with-mouse-events";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-literal-enum-members.md": {
	id: "linter/rules/use-literal-enum-members.md";
  slug: "linter/rules/use-literal-enum-members";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-literal-keys.md": {
	id: "linter/rules/use-literal-keys.md";
  slug: "linter/rules/use-literal-keys";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-media-caption.md": {
	id: "linter/rules/use-media-caption.md";
  slug: "linter/rules/use-media-caption";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-namespace-keyword.md": {
	id: "linter/rules/use-namespace-keyword.md";
  slug: "linter/rules/use-namespace-keyword";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-naming-convention.md": {
	id: "linter/rules/use-naming-convention.md";
  slug: "linter/rules/use-naming-convention";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-node-assert-strict.md": {
	id: "linter/rules/use-node-assert-strict.md";
  slug: "linter/rules/use-node-assert-strict";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-nodejs-import-protocol.md": {
	id: "linter/rules/use-nodejs-import-protocol.md";
  slug: "linter/rules/use-nodejs-import-protocol";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-number-namespace.md": {
	id: "linter/rules/use-number-namespace.md";
  slug: "linter/rules/use-number-namespace";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-numeric-literals.md": {
	id: "linter/rules/use-numeric-literals.md";
  slug: "linter/rules/use-numeric-literals";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-optional-chain.md": {
	id: "linter/rules/use-optional-chain.md";
  slug: "linter/rules/use-optional-chain";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-regex-literals.md": {
	id: "linter/rules/use-regex-literals.md";
  slug: "linter/rules/use-regex-literals";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-self-closing-elements.md": {
	id: "linter/rules/use-self-closing-elements.md";
  slug: "linter/rules/use-self-closing-elements";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-shorthand-array-type.md": {
	id: "linter/rules/use-shorthand-array-type.md";
  slug: "linter/rules/use-shorthand-array-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-shorthand-assign.md": {
	id: "linter/rules/use-shorthand-assign.md";
  slug: "linter/rules/use-shorthand-assign";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-shorthand-function-type.md": {
	id: "linter/rules/use-shorthand-function-type.md";
  slug: "linter/rules/use-shorthand-function-type";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-simple-number-keys.md": {
	id: "linter/rules/use-simple-number-keys.md";
  slug: "linter/rules/use-simple-number-keys";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-simplified-logic-expression.md": {
	id: "linter/rules/use-simplified-logic-expression.md";
  slug: "linter/rules/use-simplified-logic-expression";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-single-case-statement.md": {
	id: "linter/rules/use-single-case-statement.md";
  slug: "linter/rules/use-single-case-statement";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-single-var-declarator.md": {
	id: "linter/rules/use-single-var-declarator.md";
  slug: "linter/rules/use-single-var-declarator";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-sorted-classes.md": {
	id: "linter/rules/use-sorted-classes.md";
  slug: "linter/rules/use-sorted-classes";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-template.md": {
	id: "linter/rules/use-template.md";
  slug: "linter/rules/use-template";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-anchor.md": {
	id: "linter/rules/use-valid-anchor.md";
  slug: "linter/rules/use-valid-anchor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-aria-props.md": {
	id: "linter/rules/use-valid-aria-props.md";
  slug: "linter/rules/use-valid-aria-props";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-aria-role.md": {
	id: "linter/rules/use-valid-aria-role.md";
  slug: "linter/rules/use-valid-aria-role";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-aria-values.md": {
	id: "linter/rules/use-valid-aria-values.md";
  slug: "linter/rules/use-valid-aria-values";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-for-direction.md": {
	id: "linter/rules/use-valid-for-direction.md";
  slug: "linter/rules/use-valid-for-direction";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-lang.md": {
	id: "linter/rules/use-valid-lang.md";
  slug: "linter/rules/use-valid-lang";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-valid-typeof.md": {
	id: "linter/rules/use-valid-typeof.md";
  slug: "linter/rules/use-valid-typeof";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-while.md": {
	id: "linter/rules/use-while.md";
  slug: "linter/rules/use-while";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"linter/rules/use-yield.md": {
	id: "linter/rules/use-yield.md";
  slug: "linter/rules/use-yield";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"pt-br/404.md": {
	id: "pt-br/404.md";
  slug: "pt-br/404";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"pt-br/analyzer/index.mdx": {
	id: "pt-br/analyzer/index.mdx";
  slug: "pt-br/analyzer";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/formatter/differences-with-prettier.md": {
	id: "pt-br/formatter/differences-with-prettier.md";
  slug: "pt-br/formatter/differences-with-prettier";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"pt-br/formatter/index.mdx": {
	id: "pt-br/formatter/index.mdx";
  slug: "pt-br/formatter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/formatter/option-philosophy.md": {
	id: "pt-br/formatter/option-philosophy.md";
  slug: "pt-br/formatter/option-philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"pt-br/guides/big-projects.mdx": {
	id: "pt-br/guides/big-projects.mdx";
  slug: "pt-br/guides/big-projects";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/guides/getting-started.mdx": {
	id: "pt-br/guides/getting-started.mdx";
  slug: "pt-br/guides/getting-started";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/guides/how-biome-works.mdx": {
	id: "pt-br/guides/how-biome-works.mdx";
  slug: "pt-br/guides/how-biome-works";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/guides/integrate-in-editor.mdx": {
	id: "pt-br/guides/integrate-in-editor.mdx";
  slug: "pt-br/guides/integrate-in-editor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/guides/integrate-in-vcs.mdx": {
	id: "pt-br/guides/integrate-in-vcs.mdx";
  slug: "pt-br/guides/integrate-in-vcs";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/guides/manual-installation.mdx": {
	id: "pt-br/guides/manual-installation.mdx";
  slug: "pt-br/guides/manual-installation";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/index.mdx": {
	id: "pt-br/index.mdx";
  slug: "pt-br";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/internals/architecture.mdx": {
	id: "pt-br/internals/architecture.mdx";
  slug: "pt-br/internals/architecture";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/internals/credits.mdx": {
	id: "pt-br/internals/credits.mdx";
  slug: "pt-br/internals/credits";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/internals/language-support.mdx": {
	id: "pt-br/internals/language-support.mdx";
  slug: "pt-br/internals/language-support";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/internals/philosophy.mdx": {
	id: "pt-br/internals/philosophy.mdx";
  slug: "pt-br/internals/philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/internals/versioning.mdx": {
	id: "pt-br/internals/versioning.mdx";
  slug: "pt-br/internals/versioning";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/linter/index.mdx": {
	id: "pt-br/linter/index.mdx";
  slug: "pt-br/linter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/recipes/continuous-integration.mdx": {
	id: "pt-br/recipes/continuous-integration.mdx";
  slug: "pt-br/recipes/continuous-integration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/recipes/git-hooks.mdx": {
	id: "pt-br/recipes/git-hooks.mdx";
  slug: "pt-br/recipes/git-hooks";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"pt-br/reference/vscode.mdx": {
	id: "pt-br/reference/vscode.mdx";
  slug: "pt-br/reference/vscode";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"recipes/continuous-integration.mdx": {
	id: "recipes/continuous-integration.mdx";
  slug: "recipes/continuous-integration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"recipes/git-hooks.mdx": {
	id: "recipes/git-hooks.mdx";
  slug: "recipes/git-hooks";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"reference/cli.mdx": {
	id: "reference/cli.mdx";
  slug: "reference/cli";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"reference/configuration.mdx": {
	id: "reference/configuration.mdx";
  slug: "reference/configuration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"reference/configuration_example.mdx": {
	id: "reference/configuration_example.mdx";
  slug: "reference/configuration_example";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"reference/vscode.mdx": {
	id: "reference/vscode.mdx";
  slug: "reference/vscode";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/analyzer/index.mdx": {
	id: "zh-cn/analyzer/index.mdx";
  slug: "zh-cn/analyzer";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/formatter/differences-with-prettier.md": {
	id: "zh-cn/formatter/differences-with-prettier.md";
  slug: "zh-cn/formatter/differences-with-prettier";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"zh-cn/formatter/index.mdx": {
	id: "zh-cn/formatter/index.mdx";
  slug: "zh-cn/formatter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/formatter/option-philosophy.md": {
	id: "zh-cn/formatter/option-philosophy.md";
  slug: "zh-cn/formatter/option-philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".md"] };
"zh-cn/guides/big-projects.mdx": {
	id: "zh-cn/guides/big-projects.mdx";
  slug: "zh-cn/guides/big-projects";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/guides/getting-started.mdx": {
	id: "zh-cn/guides/getting-started.mdx";
  slug: "zh-cn/guides/getting-started";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/guides/how-biome-works.mdx": {
	id: "zh-cn/guides/how-biome-works.mdx";
  slug: "zh-cn/guides/how-biome-works";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/guides/integrate-in-editor.mdx": {
	id: "zh-cn/guides/integrate-in-editor.mdx";
  slug: "zh-cn/guides/integrate-in-editor";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/guides/integrate-in-vcs.mdx": {
	id: "zh-cn/guides/integrate-in-vcs.mdx";
  slug: "zh-cn/guides/integrate-in-vcs";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/guides/manual-installation.mdx": {
	id: "zh-cn/guides/manual-installation.mdx";
  slug: "zh-cn/guides/manual-installation";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/index.mdx": {
	id: "zh-cn/index.mdx";
  slug: "zh-cn";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/architecture.mdx": {
	id: "zh-cn/internals/architecture.mdx";
  slug: "zh-cn/internals/architecture";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/changelog.mdx": {
	id: "zh-cn/internals/changelog.mdx";
  slug: "zh-cn/internals/changelog";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/credits.mdx": {
	id: "zh-cn/internals/credits.mdx";
  slug: "zh-cn/internals/credits";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/language-support.mdx": {
	id: "zh-cn/internals/language-support.mdx";
  slug: "zh-cn/internals/language-support";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/philosophy.mdx": {
	id: "zh-cn/internals/philosophy.mdx";
  slug: "zh-cn/internals/philosophy";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/internals/versioning.mdx": {
	id: "zh-cn/internals/versioning.mdx";
  slug: "zh-cn/internals/versioning";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/linter/index.mdx": {
	id: "zh-cn/linter/index.mdx";
  slug: "zh-cn/linter";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/recipes/continuous-integration.mdx": {
	id: "zh-cn/recipes/continuous-integration.mdx";
  slug: "zh-cn/recipes/continuous-integration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/recipes/git-hooks.mdx": {
	id: "zh-cn/recipes/git-hooks.mdx";
  slug: "zh-cn/recipes/git-hooks";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/reference/configuration.mdx": {
	id: "zh-cn/reference/configuration.mdx";
  slug: "zh-cn/reference/configuration";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
"zh-cn/reference/vscode.mdx": {
	id: "zh-cn/reference/vscode.mdx";
  slug: "zh-cn/reference/vscode";
  body: string;
  collection: "docs";
  data: InferEntrySchema<"docs">
} & { render(): Render[".mdx"] };
};

	};

	type DataEntryMap = {
		"authors": {
"core": {
	id: "core";
  collection: "authors";
  data: InferEntrySchema<"authors">
};
"ema": {
	id: "ema";
  collection: "authors";
  data: InferEntrySchema<"authors">
};
"team": {
	id: "team";
  collection: "authors";
  data: InferEntrySchema<"authors">
};
};

	};

	type AnyEntryMap = ContentEntryMap & DataEntryMap;

	export type ContentConfig = typeof import("../src/content/config.js");
}
