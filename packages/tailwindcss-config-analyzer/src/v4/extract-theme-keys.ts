// Extract per-namespace theme keys from Tailwind v4's default theme.css.
//
// For every prefix listed in `THEME_NAMESPACES`, scan
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/theme.css
// for declarations of the form `--<prefix><key>: <value>;` and collect the
// `<key>` strings.
//
// Companion declarations like `--text-lg--line-height` are skipped (they
// share a prefix but the key contains `--`, signalling a sub-token rather
// than a top-level theme key).

import fs from "node:fs/promises";
import { createRequire } from "node:module";
import path from "node:path";
import {
	THEME_NAMESPACES,
	type ThemeNamespacePrefix,
} from "./theme-namespaces.js";

export type ThemeKeysByPrefix = Map<ThemeNamespacePrefix, Set<string>>;

export async function extractThemeKeys(): Promise<ThemeKeysByPrefix> {
	const require = createRequire(import.meta.url);
	const tailwindRoot = path.dirname(
		require.resolve("tailwindcss/package.json"),
	);
	const themeCss = await fs.readFile(
		path.join(tailwindRoot, "theme.css"),
		"utf8",
	);

	const result: ThemeKeysByPrefix = new Map();

	// Sort prefixes by length descending so e.g. `--text-shadow-` matches
	// before `--text-` when scanning a single line.
	const prefixes = THEME_NAMESPACES.map((n) => n.cssPrefix).sort(
		(a, b) => b.length - a.length,
	);

	for (const line of themeCss.split("\n")) {
		const m = line.match(/^\s*(--[a-z][a-z0-9-]*):/);
		if (!m) continue;
		const decl = m[1];
		for (const prefix of prefixes) {
			if (!decl.startsWith(prefix)) continue;
			const key = decl.slice(prefix.length);
			if (key.length === 0) break; // bare `--color-:` shouldn't happen
			if (key.includes("--")) break; // companion (e.g. lg--line-height)
			let set = result.get(prefix as ThemeNamespacePrefix);
			if (!set) {
				set = new Set();
				result.set(prefix as ThemeNamespacePrefix, set);
			}
			set.add(key);
			break;
		}
	}

	return result;
}
