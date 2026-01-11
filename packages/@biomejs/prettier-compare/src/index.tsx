/**
 * CLI entry point for prettier-compare.
 *
 * Usage:
 *   prettier-compare "const x = 1"           # Format a snippet
 *   prettier-compare -f file.ts              # Format from file
 *   echo "const x = 1" | prettier-compare    # Format from stdin
 *   prettier-compare -w "const x = 1"        # Watch mode
 */

import { program } from "commander";
import { createCliRenderer } from "@opentui/core";
import { createRoot } from "@opentui/react";
import { readFileSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import React from "react";
import { App } from "./components/App.js";
import { detectLanguage, getSupportedLanguages } from "./languages.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Find biome repo root (package is in packages/@biomejs/prettier-compare)
const ROOT_DIR = resolve(__dirname, "../../../..");

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

program
	.name("prettier-compare")
	.description(
		"Compare Biome and Prettier formatting output and IR side-by-side",
	)
	.argument("[snippet]", "Code snippet to format")
	.option("-f, --file <path>", "Read input from file")
	.option(
		"-l, --language <lang>",
		`Language (${getSupportedLanguages().slice(0, 8).join(", ")}, ...)`,
	)
	.option("-w, --watch", "Watch mode: rebuild WASM on Rust file changes")
	.option("-r, --rebuild", "Rebuild WASM before running")
	.option("--ir-only", "Only show IR comparison, not formatted output")
	.option("--output-only", "Only show formatted output, not IR")
	.action(async (snippet, options) => {
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
		} else if (snippet) {
			// Use provided snippet
			code = snippet;
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

		// Create the TUI renderer
		const renderer = await createCliRenderer({
			targetFps: options.watch ? 30 : 1,
		});

		const handleExit = () => {
			renderer.stop();
			process.exit(0);
		};

		// Create React root and render the app
		const root = createRoot(renderer);
		root.render(
			<App
				code={code}
				language={language}
				watchMode={options.watch ?? false}
				rootDir={ROOT_DIR}
				onExit={handleExit}
				irOnly={options.irOnly}
				outputOnly={options.outputOnly}
				rebuild={options.rebuild}
			/>,
		);

		if (options.watch) {
			// Interactive watch mode
			renderer.start();

			// Handle Ctrl+C
			process.on("SIGINT", handleExit);
			process.on("SIGTERM", handleExit);
		} else {
			// Non-interactive: render once and exit after async work completes
			// We need a small delay to let React effects run and async formatting complete
			setTimeout(() => {
				// Re-render to show final state
				setTimeout(() => {
					renderer.stop();
					process.exit(0);
				}, 100);
			}, 500);
		}
	});

program.parse();
