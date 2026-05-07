// Entry point for the Tailwind v4 preset codegen.
//
// Usage:
//   pnpm --filter tailwindcss-config-analyzer execute:v4
//
// Output:
//   crates/biome_js_analyze/src/lint/nursery/use_sorted_classes/tailwind_preset_v4.rs

import { spawn } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { readPackageUp } from "read-package-up";

import { extractPropertyOrder } from "./extract-property-order.js";
import {
	extractThemeKeys,
	type ThemeKeysByPrefix,
} from "./extract-theme-keys.js";
import { extractUtilities } from "./extract-utilities.js";
import { renderRust } from "./render-rust.js";
import { THEME_NAMESPACES } from "./theme-namespaces.js";

const ROOT_PACKAGE_NAME = "@biomejs/monorepo";
const OUTPUT_PATH =
	"crates/biome_js_analyze/src/lint/nursery/use_sorted_classes/tailwind_preset_v4.rs";

async function findRepoRoot(): Promise<string> {
	let cwd = path.dirname(fileURLToPath(import.meta.url));
	while (true) {
		const pkg = await readPackageUp({ cwd });
		if (!pkg) throw new Error("could not locate @biomejs/monorepo root");
		if (pkg.packageJson.name === ROOT_PACKAGE_NAME)
			return path.dirname(pkg.path);
		cwd = path.resolve(path.dirname(pkg.path), "..");
	}
}

function verifyNamespaces(themeKeys: ThemeKeysByPrefix): void {
	const catalog = new Set<string>(THEME_NAMESPACES.map((n) => n.cssPrefix));
	const observed = new Set<string>(themeKeys.keys());

	const unexpected: string[] = [];
	for (const prefix of observed) {
		if (!catalog.has(prefix)) unexpected.push(prefix);
	}
	if (unexpected.length > 0) {
		throw new Error(
			`theme.css contains namespace prefixes missing from the catalog: ${unexpected.join(", ")}\n` +
				"Add them to packages/tailwindcss-config-analyzer/src/v4/theme-namespaces.ts.",
		);
	}

	// Catalog can legitimately have entries with no keys (e.g. Spacing,
	// BackgroundImage in the default theme). Just report informationally.
	const empty: string[] = [];
	for (const prefix of catalog) {
		if (!observed.has(prefix)) empty.push(prefix);
	}
	if (empty.length > 0) {
		console.warn(
			`note: catalog namespaces with no keys in default theme.css: ${empty.join(", ")}`,
		);
	}
}

function runRustfmt(filePath: string): Promise<void> {
	return new Promise((resolve) => {
		const proc = spawn("rustfmt", ["--edition=2024", filePath], {
			stdio: ["ignore", "inherit", "inherit"],
		});
		proc.on("error", (err) => {
			console.warn(
				`note: skipped rustfmt (${err.message}). install rustfmt to format the output.`,
			);
			resolve();
		});
		proc.on("close", (code) => {
			if (code !== 0) {
				console.warn(
					`note: rustfmt exited with code ${code}; output left unformatted.`,
				);
			}
			resolve();
		});
	});
}

async function main() {
	const [propertyOrder, themeKeys, utilities] = await Promise.all([
		extractPropertyOrder(),
		extractThemeKeys(),
		extractUtilities(),
	]);

	verifyNamespaces(themeKeys);

	const rust = renderRust({ propertyOrder, themeKeys, utilities });

	const repoRoot = await findRepoRoot();
	const outPath = path.join(repoRoot, OUTPUT_PATH);
	await fs.writeFile(outPath, rust);

	await runRustfmt(outPath);

	console.log(`wrote ${OUTPUT_PATH}`);
	console.log(
		`  property-order: ${propertyOrder.length}, namespaces with keys: ${themeKeys.size}, ` +
			`static: ${utilities.static.length}, functional: ${utilities.functional.length}`,
	);
}

await main();
