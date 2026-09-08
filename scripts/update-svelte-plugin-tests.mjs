#!/usr/bin/env node
// Imports the test suite of `prettier-plugin-svelte` into
// `crates/biome_html_formatter/tests/specs/svelte-plugin/`.
//
// The plugin keeps two suites, and both become the same pair of files that the
// rest of our Prettier comparison tests use: an input, and a `.prettier-snap`
// holding what Prettier produces for it.
//
//   test/printer/samples/x.html      already-formatted, so it is its own output
//   test/formatting/samples/x/       input.html and output.html
//
// Usage: node scripts/update-svelte-plugin-tests.mjs [--ref <git ref>]

import { execFileSync } from "node:child_process";
import {
	existsSync,
	mkdirSync,
	mkdtempSync,
	readdirSync,
	readFileSync,
	rmSync,
	writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPOSITORY = "https://github.com/sveltejs/prettier-plugin-svelte.git";
const DEFAULT_REF = "prettier-plugin-svelte@4.1.1";

const repositoryRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const destination = join(
	repositoryRoot,
	"crates/biome_html_formatter/tests/specs/svelte-plugin",
);

const refIndex = process.argv.indexOf("--ref");
const ref = refIndex === -1 ? DEFAULT_REF : process.argv[refIndex + 1];

const checkout = mkdtempSync(join(tmpdir(), "prettier-plugin-svelte-"));
try {
	execFileSync(
		"git",
		["clone", "--quiet", "--depth", "1", "--branch", ref, REPOSITORY, checkout],
		{
			stdio: ["ignore", "ignore", "inherit"],
		},
	);

	rmSync(destination, { recursive: true, force: true });

	const printer = importPrinterSamples(join(checkout, "test/printer/samples"));
	const formatting = importFormattingSamples(
		join(checkout, "test/formatting/samples"),
	);

	writeFileSync(
		join(destination, "README.md"),
		`# prettier-plugin-svelte

Imported from ${REPOSITORY} at \`${ref}\` by \`scripts/update-svelte-plugin-tests.mjs\`.
Do not edit these by hand; re-run the script instead.

- \`printer/\`: samples the plugin expects to be left untouched, so the input is
  also the expected output.
- \`formatting/\`: samples with a separate input and expected output.

A \`.options.json\` next to a sample holds the plugin options that sample runs
with. Options we cannot express are ignored, and show up as a difference.
`,
	);

	console.log(
		`Imported ${printer} printer samples and ${formatting} formatting samples from ${ref}.`,
	);
} finally {
	rmSync(checkout, { recursive: true, force: true });
}

/** Normalizes line endings the way both of the plugin's test drivers do. */
function read(path) {
	return readFileSync(path, "utf8").replaceAll(/\r?\n/g, "\n");
}

function write(directory, name, contents) {
	mkdirSync(directory, { recursive: true });
	writeFileSync(join(directory, name), contents);
}

function importPrinterSamples(source) {
	const target = join(destination, "printer");
	let imported = 0;

	for (const file of readdirSync(source).sort()) {
		// `.md` samples go through Prettier's markdown printer, and `.skip` and
		// `.only` mark samples the plugin's own suite is not running.
		if (
			!file.endsWith(".html") ||
			file.includes(".skip") ||
			file.includes(".only")
		) {
			continue;
		}

		const name = file.slice(0, -".html".length);
		const contents = read(join(source, file));

		write(target, `${name}.svelte`, contents);
		write(target, `${name}.svelte.prettier-snap`, contents);

		const options = join(source, `${name}.options.json`);
		if (existsSync(options)) {
			write(target, `${name}.options.json`, read(options));
		}

		imported += 1;
	}

	return imported;
}

function importFormattingSamples(source) {
	const target = join(destination, "formatting");
	let imported = 0;

	for (const name of readdirSync(source).sort()) {
		if (name.includes(".skip") || name.includes(".only")) {
			continue;
		}

		const directory = join(source, name);
		write(target, `${name}.svelte`, read(join(directory, "input.html")));
		write(
			target,
			`${name}.svelte.prettier-snap`,
			read(join(directory, "output.html")),
		);

		const options = join(directory, "options.json");
		if (existsSync(options)) {
			write(target, `${name}.options.json`, read(options));
		}

		imported += 1;
	}

	return imported;
}
