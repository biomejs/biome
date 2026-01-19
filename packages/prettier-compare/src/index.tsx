/**
 * CLI entry point for prettier-compare.
 *
 * Usage:
 *   prettier-compare "const x = 1"           # Format a snippet
 *   prettier-compare -f file.ts              # Format from file
 *   echo "const x = 1" | prettier-compare    # Format from stdin
 *   prettier-compare -w "const x = 1"        # Watch mode (fancy TUI)
 */

import { parseArgs } from "node:util";
import { readFileSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import {
	detectLanguage,
	getSupportedLanguages,
	getLanguageConfig,
} from "./languages.js";
import { formatWithBiome } from "./biome.js";
import { formatWithPrettier } from "./prettier.js";
import { rebuildWasm } from "./watch.js";
import { printComparison } from "./plainOutput.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Find biome repo root (package is in packages/prettier-compare)
const ROOT_DIR = resolve(__dirname, "../../..");

/**
 * Read all input from stdin.
 */
async function readStdin(): Promise<string> {
	const chunks: Buffer[] = [];
	for await (const chunk of process.stdin) {
		chunks.push(chunk as Buffer);
	}
	return Buffer.concat(chunks).toString("utf-8");
}

type RawCliOptionValues = {
	file?: string;
	language?: string;
	watch?: boolean;
	rebuild?: boolean;
	"ir-only"?: boolean;
	"output-only"?: boolean;
	help?: boolean;
};

type NormalizedCliOptions = {
	file?: string;
	language?: string;
	watch: boolean;
	rebuild: boolean;
	irOnly: boolean;
	outputOnly: boolean;
};

function printHelp() {
	const languages = getSupportedLanguages().slice(0, 8).join(", ");

	console.info(
		"Compare Biome and Prettier formatting output and IR side-by-side.\n",
	);
	console.info("Usage:");
	console.info('  prettier-compare "const x = 1"           # Format a snippet');
	console.info("  prettier-compare -f file.ts              # Format from file");
	console.info('  echo "const x = 1" | prettier-compare    # Format from stdin');
	console.info('  prettier-compare -w "const x = 1"        # Watch mode (fancy TUI)');
	console.info("\nOptions:");
	console.info("  -f, --file <path>        Read input from file");
	console.info(
		`  -l, --language <lang>    Language (${languages}, ...)`,
	);
	console.info(
		"  -w, --watch              Watch mode: rebuild WASM on Rust file changes",
	);
	console.info("  -r, --rebuild            Rebuild WASM before running");
	console.info(
		"      --ir-only            Only show IR comparison, not formatted output",
	);
	console.info("      --output-only        Only show formatted output, not IR");
	console.info("  -h, --help               Show this help message");
}

function parseCliArgs(): { snippet?: string; options: NormalizedCliOptions } {
	const { values, positionals } = parseArgs({
		args: process.argv.slice(2),
		allowPositionals: true,
		options: {
			file: { type: "string", short: "f" },
			language: { type: "string", short: "l" },
			watch: { type: "boolean", short: "w", default: false },
			rebuild: { type: "boolean", short: "r", default: false },
			"ir-only": { type: "boolean", default: false },
			"output-only": { type: "boolean", default: false },
			help: { type: "boolean", short: "h", default: false },
		},
	}) as {
		values: RawCliOptionValues;
		positionals: string[];
	};

	if (values.help) {
		printHelp();
		process.exit(0);
	}

	return {
		snippet: positionals[0],
		options: {
			file: values.file,
			language: values.language,
			watch: values.watch ?? false,
			rebuild: values.rebuild ?? false,
			irOnly: values["ir-only"] ?? false,
			outputOnly: values["output-only"] ?? false,
		},
	};
}

async function run() {
	const {
		snippet: snippetArg,
		options,
	} = parseCliArgs();

	// Determine input source
	let code: string;
	let detectedLang: string | undefined;

	if (options.file) {
		// Read from file
		try {
			code = readFileSync(options.file, "utf-8");
			detectedLang = detectLanguage(options.file);
		} catch (err) {
			console.error(
				`Error reading file: ${err instanceof Error ? err.message : err}`,
			);
			process.exit(1);
		}
	} else if (snippetArg) {
		// Use provided snippet
		code = snippetArg;
	} else if (!process.stdin.isTTY) {
		// Read from stdin
		code = await readStdin();
	} else {
		// No input provided
		console.error(
			"Error: No input provided. Pass a snippet, use --file, or pipe to stdin.",
		);
		console.error("");
		console.error("Examples:");
		console.error('  prettier-compare "const x = { a: 1 }"');
		console.error("  prettier-compare -f src/example.ts");
		console.error("  echo 'const x = 1' | prettier-compare");
		process.exit(1);
	}

	const language = options.language ?? detectedLang ?? "js";

	if (options.watch) {
		// Watch mode: Use fancy TUI with React/OpenTUI
		const { createCliRenderer } = await import("@opentui/core");
		const { createRoot } = await import("@opentui/react");
		const React = await import("react");
		const { App } = await import("./components/App.js");

		const renderer = await createCliRenderer({
			targetFps: 30,
		});

		const handleExit = () => {
			renderer.stop();
			process.exit(0);
		};

		const root = createRoot(renderer);
		root.render(
			React.createElement(App, {
				code,
				language,
				watchMode: true,
				rootDir: ROOT_DIR,
				onExit: handleExit,
				irOnly: options.irOnly,
				outputOnly: options.outputOnly,
				rebuild: options.rebuild,
			}),
		);

		renderer.start();

		// Handle Ctrl+C
		process.on("SIGINT", handleExit);
		process.on("SIGTERM", handleExit);
	} else {
		// Non-watch mode: Plain sequential output to stdout
		const config = getLanguageConfig(language);

		// Optionally rebuild WASM first
		if (options.rebuild) {
			console.info("Rebuilding WASM...");
			try {
				await rebuildWasm(ROOT_DIR);
				console.info("WASM rebuilt successfully.\n");
			} catch (err) {
				console.error(
					`WASM rebuild failed: ${err instanceof Error ? err.message : err}`,
				);
				process.exit(1);
			}
		}

		// Run formatting
		try {
			const [biomeResult, prettierResult] = await Promise.all([
				formatWithBiome(code, config.biomeFilePath),
				formatWithPrettier(code, config.prettierParser),
			]);

			printComparison({
				biomeResult,
				prettierResult,
				language,
				irOnly: options.irOnly,
				outputOnly: options.outputOnly,
			});
		} catch (err) {
			console.error(
				`Formatting failed: ${err instanceof Error ? err.message : err}`,
			);
			process.exit(1);
		}
	}
}

run().catch((err) => {
	console.error(
		err instanceof Error ? err.stack ?? err.message : String(err),
	);
	process.exit(1);
});
