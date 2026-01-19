/**
 * Plain text output for non-watch mode.
 * Prints comparison results sequentially to stdout with basic ANSI colors.
 */

import { styleText } from "node:util";

import type { BiomeResult } from "./biome.js";
import type { PrettierResult } from "./prettier.js";
import { getLanguageConfig } from "./languages.js";

function header(text: string): string {
	return styleText(["bold", "cyan"], text);
}

function matchIndicator(isMatch: boolean): string {
	if (isMatch) {
		return styleText("green", "[MATCH]");
	}
	return styleText("yellow", "[DIFF]");
}

interface PrintComparisonOptions {
	biomeResult: BiomeResult;
	prettierResult: PrettierResult;
	language: string;
	irOnly?: boolean;
	outputOnly?: boolean;
}

/**
 * Print comparison results sequentially to stdout.
 */
export function printComparison({
	biomeResult,
	prettierResult,
	language,
	irOnly = false,
	outputOnly = false,
}: PrintComparisonOptions): void {
	const outputMatch = biomeResult.output === prettierResult.output;
	const config = getLanguageConfig(language);

	// Formatted output comparison
	if (!irOnly) {
		console.info(
			`${header("=== Formatted Output ===")}  ${matchIndicator(outputMatch)}`,
		);
		console.info();

		console.info(styleText("cyan", "--- Biome ---"));
		console.info(biomeResult.output);

		console.info(styleText("magenta", "--- Prettier ---"));
		console.info(prettierResult.output);
	}

	// IR comparison
	if (!outputOnly) {
		console.info(header("=== IR (Intermediate Representation) ==="));
		console.info();

		console.info(styleText("cyan", "--- Biome ---"));
		console.info(biomeResult.ir || "(no IR available)");
		console.info();

		console.info(styleText("magenta", "--- Prettier ---"));
		console.info(prettierResult.ir || "(no IR available)");
		console.info();
	}

	// Diagnostics
	const hasDiagnostics =
		biomeResult.diagnostics.length > 0 || prettierResult.error;

	if (hasDiagnostics) {
		console.info(header("=== Diagnostics ==="));
		console.info();

		if (biomeResult.diagnostics.length > 0) {
			console.info(styleText("cyan", "Biome:"));
			for (const d of biomeResult.diagnostics) {
				const severityStyle = d.severity === "error" ? "red" : "yellow";
				console.info(
					`  ${styleText(severityStyle, `[${d.severity}]`)} ${d.description}`,
				);
			}
		}

		if (prettierResult.error) {
			console.info(styleText("magenta", "Prettier:"));
			console.info(
				`  ${styleText("red", "[error]")} ${prettierResult.error}`,
			);
		}

		console.info();
	}

	// Language info
	console.info(
		styleText(
			"gray",
			`Language: ${config.displayName} | Prettier parser: ${config.prettierParser}`,
		),
	);
}
