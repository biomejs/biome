// Theme-namespace catalog — single source of truth.
//
// Mirrors the `--<namespace>-*` prefixes shipped in
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/theme.css
//
// Used by the codegen script to:
//   - emit `pub enum ThemeNamespace { ... }` into the generated Rust file
//   - emit `impl ThemeNamespace { fn css_prefix(...) }` from `cssPrefix`
//   - drive theme-key extraction (one phf::Set per entry)
//   - inject probe tokens into namespaces while inferring utility branches
//
// `verify-namespaces.ts` runs at codegen time and fails if this list drifts
// from the prefixes actually present in `theme.css`.

export const THEME_NAMESPACES = [
	{ variant: "Color", cssPrefix: "--color-" },
	{ variant: "Spacing", cssPrefix: "--spacing-" },
	{ variant: "Text", cssPrefix: "--text-" },
	{ variant: "TextShadow", cssPrefix: "--text-shadow-" },
	{ variant: "Font", cssPrefix: "--font-" },
	{ variant: "FontWeight", cssPrefix: "--font-weight-" },
	{ variant: "Leading", cssPrefix: "--leading-" },
	{ variant: "Tracking", cssPrefix: "--tracking-" },
	{ variant: "Breakpoint", cssPrefix: "--breakpoint-" },
	{ variant: "Container", cssPrefix: "--container-" },
	{ variant: "Radius", cssPrefix: "--radius-" },
	{ variant: "Shadow", cssPrefix: "--shadow-" },
	{ variant: "InsetShadow", cssPrefix: "--inset-shadow-" },
	{ variant: "DropShadow", cssPrefix: "--drop-shadow-" },
	{ variant: "Blur", cssPrefix: "--blur-" },
	{ variant: "Perspective", cssPrefix: "--perspective-" },
	{ variant: "Aspect", cssPrefix: "--aspect-" },
	{ variant: "Ease", cssPrefix: "--ease-" },
	{ variant: "Animate", cssPrefix: "--animate-" },
	{ variant: "BackgroundImage", cssPrefix: "--background-image-" },
] as const;

export type ThemeNamespaceVariant =
	(typeof THEME_NAMESPACES)[number]["variant"];
export type ThemeNamespacePrefix =
	(typeof THEME_NAMESPACES)[number]["cssPrefix"];

export function variantOf(prefix: string): ThemeNamespaceVariant | undefined {
	return THEME_NAMESPACES.find((n) => n.cssPrefix === prefix)?.variant;
}

export function prefixOf(
	variant: ThemeNamespaceVariant,
): ThemeNamespacePrefix | undefined {
	return THEME_NAMESPACES.find((n) => n.variant === variant)?.cssPrefix;
}
