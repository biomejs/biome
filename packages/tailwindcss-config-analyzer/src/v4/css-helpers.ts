// Shared helpers for parsing compiled-utility CSS and loading the
// Tailwind v4 design system.

import fs from "node:fs/promises";
import { createRequire } from "node:module";
import path from "node:path";

const META_PROPERTIES = new Set(["syntax", "inherits", "initial-value"]);
const DECL_RE = /^\s*(-{0,2}[a-zA-Z][a-zA-Z0-9-]*)\s*:\s*[^;\n]+;/gm;
const TW_SORT_RE = /--tw-sort:\s*([a-zA-Z-]+)/;

export type ParsedCss = {
	sort_property: string | null;
	property_count: number;
};

// Single-pass parser — first non-meta declaration as sort property,
// non-meta declaration count for tie-breaking. `--tw-*` properties
// are intentionally NOT skipped: in v4 they appear in
// `property-order.ts` and are sort-relevant.
export function parseDeclarations(css: string): ParsedCss {
	let sort_property: string | null = null;
	let property_count = 0;

	const hint = css.match(TW_SORT_RE);
	if (hint) sort_property = hint[1];

	for (const m of css.matchAll(DECL_RE)) {
		const prop = m[1];
		if (META_PROPERTIES.has(prop)) continue;
		if (sort_property === null) sort_property = prop;
		property_count++;
	}

	return { sort_property, property_count };
}

// Build a `loadStylesheet` resolver bound to the installed
// `tailwindcss` package directory. Handles `@import "tailwindcss"`,
// `@import "tailwindcss/<sub>"`, and relative imports.
export function makeLoadStylesheet() {
	const require = createRequire(import.meta.url);
	const tailwindRoot = path.dirname(
		require.resolve("tailwindcss/package.json"),
	);
	return async function loadStylesheet(id: string, base: string) {
		let resolved: string;
		if (id === "tailwindcss") resolved = path.join(tailwindRoot, "index.css");
		else if (id.startsWith("tailwindcss/")) {
			const sub = id.slice("tailwindcss/".length);
			resolved = path.join(
				tailwindRoot,
				sub.endsWith(".css") ? sub : `${sub}.css`,
			);
		} else if (id.startsWith(".")) {
			resolved = path.resolve(base, id);
		} else {
			resolved = require.resolve(id, { paths: [base] });
		}
		const content = await fs.readFile(resolved, "utf8");
		return { path: resolved, content, base: path.dirname(resolved) };
	};
}
