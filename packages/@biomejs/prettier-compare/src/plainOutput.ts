/**
 * Plain text output for non-watch mode.
 * Prints comparison results sequentially to stdout with basic ANSI colors.
 */

import type { BiomeResult } from "./biome.js";
import type { PrettierResult } from "./prettier.js";
import { getLanguageConfig } from "./languages.js";

// ANSI color codes
const colors = {
	reset: "\x1b[0m",
	bold: "\x1b[1m",
	cyan: "\x1b[36m",
	green: "\x1b[32m",
	yellow: "\x1b[33m",
	red: "\x1b[31m",
	gray: "\x1b[90m",
	magenta: "\x1b[35m",
};

function header(text: string): string {
	return `${colors.bold}${colors.cyan}${text}${colors.reset}`;
}

function subHeader(text: string, color: string): string {
	return `${color}${text}${colors.reset}`;
}

function matchIndicator(isMatch: boolean): string {
	if (isMatch) {
		return `${colors.green}[MATCH]${colors.reset}`;
	}
	return `${colors.yellow}[DIFF]${colors.reset}`;
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
		console.log(
			`${header("=== Formatted Output ===")}  ${matchIndicator(outputMatch)}`,
		);
		console.log();

		console.log(subHeader("--- Biome ---", colors.cyan));
		console.log(biomeResult.output);

		console.log(subHeader("--- Prettier ---", colors.magenta));
		console.log(prettierResult.output);
	}

	// IR comparison
	if (!outputOnly) {
		console.log(header("=== IR (Intermediate Representation) ==="));
		console.log();

		console.log(subHeader("--- Biome ---", colors.cyan));
		console.log(biomeResult.ir || "(no IR available)");
		console.log();

		console.log(subHeader("--- Prettier ---", colors.magenta));
		console.log(prettierResult.ir || "(no IR available)");
		console.log();
	}

	// Diagnostics
	const hasDiagnostics =
		biomeResult.diagnostics.length > 0 || prettierResult.error;

	if (hasDiagnostics) {
		console.log(header("=== Diagnostics ==="));
		console.log();

		if (biomeResult.diagnostics.length > 0) {
			console.log(subHeader("Biome:", colors.cyan));
			for (const d of biomeResult.diagnostics) {
				const color = d.severity === "error" ? colors.red : colors.yellow;
				console.log(
					`  ${color}[${d.severity}]${colors.reset} ${d.description}`,
				);
			}
		}

		if (prettierResult.error) {
			console.log(subHeader("Prettier:", colors.magenta));
			console.log(
				`  ${colors.red}[error]${colors.reset} ${prettierResult.error}`,
			);
		}

		console.log();
	}

	// Language info
	console.log(
		`${colors.gray}Language: ${config.displayName} | Prettier parser: ${config.prettierParser}${colors.reset}`,
	);
}
