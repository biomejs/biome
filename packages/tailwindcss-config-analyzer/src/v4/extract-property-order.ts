// Extract the CSS property sort order from a Tailwind v4 npm bundle.
//
// The list lives in
// https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/src/property-order.ts
// but only ships in minified form. We locate it by anchoring on a
// well-known sequence ("padding","padding-inline") and walking outward
// to the enclosing `[ ... ]` literal, which is then JSON-parsed.

import fs from "node:fs/promises";
import { createRequire } from "node:module";
import path from "node:path";

const ANCHOR = '"padding","padding-inline"';

export async function extractPropertyOrder(): Promise<string[]> {
	const require = createRequire(import.meta.url);
	const tailwindRoot = path.dirname(
		require.resolve("tailwindcss/package.json"),
	);
	const distDir = path.join(tailwindRoot, "dist");
	const chunkFiles = (await fs.readdir(distDir)).filter(
		(f) => f.startsWith("chunk-") && f.endsWith(".mjs"),
	);

	for (const file of chunkFiles) {
		const content = await fs.readFile(path.join(distDir, file), "utf8");
		const anchorIdx = content.indexOf(ANCHOR);
		if (anchorIdx === -1) continue;

		const start = content.lastIndexOf("[", anchorIdx);
		const end = content.indexOf("]", anchorIdx);
		if (start === -1 || end === -1) {
			throw new Error(
				`property-order anchor found in ${file} but cannot locate enclosing brackets`,
			);
		}

		const literal = content.slice(start, end + 1);
		const parsed = JSON.parse(literal);
		if (!Array.isArray(parsed) || parsed.some((x) => typeof x !== "string")) {
			throw new Error(
				`property-order literal in ${file} is not a string array`,
			);
		}
		return parsed as string[];
	}

	throw new Error(
		`property-order array not found in any tailwindcss/dist/chunk-*.mjs (anchor: ${ANCHOR})`,
	);
}
