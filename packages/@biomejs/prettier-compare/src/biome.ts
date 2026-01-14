/**
 * Biome formatting integration using @biomejs/js-api
 */

import type { Biome as BiomeType } from "@biomejs/js-api";

export interface BiomeDiagnostic {
	description: string;
	severity: string;
}

export interface BiomeResult {
	/** The formatted output */
	output: string;
	/** The formatter IR (intermediate representation) */
	ir: string;
	/** Any diagnostics/errors encountered */
	diagnostics: BiomeDiagnostic[];
}

/** Enable debug logging via environment variable */
const DEBUG = process.env.DEBUG_BIOME === "1" || process.env.DEBUG === "1";

function debug(...args: unknown[]) {
	if (DEBUG) {
		console.log("[biome]", ...args);
	}
}

let biomeInstance: BiomeType | null = null;

/**
 * Dynamically import @biomejs/js-api to allow cache busting for reloads.
 * We use a cache-busting query parameter to force Bun to reimport the module.
 */
async function importBiomeApi(cacheBuster?: number) {
	// Bun supports cache-busting query params on dynamic imports
	const importPath = cacheBuster
		? `@biomejs/js-api?v=${cacheBuster}`
		: "@biomejs/js-api";

	debug("Importing Biome API from:", importPath);

	const module = await import(/* @vite-ignore */ importPath);
	return module as typeof import("@biomejs/js-api");
}

/**
 * Get or create a Biome instance.
 */
async function getBiome(): Promise<BiomeType> {
	if (!biomeInstance) {
		debug("Creating new Biome instance...");
		const { Biome, Distribution } = await importBiomeApi();
		biomeInstance = await Biome.create({ distribution: Distribution.NODE });
		debug("Biome instance created");
	}
	return biomeInstance;
}

/**
 * Reload the Biome instance (used after WASM rebuild).
 * This shuts down the current instance and forces a fresh import of the WASM module.
 */
export async function reloadBiome(): Promise<void> {
	debug("Reloading Biome...");

	if (biomeInstance) {
		debug("Shutting down existing instance");
		biomeInstance.shutdown();
		biomeInstance = null;
	}

	// Use a cache-busting timestamp to force Bun to reimport the module
	// This ensures the newly built WASM is loaded
	const cacheBuster = Date.now();
	const { Biome, Distribution } = await importBiomeApi(cacheBuster);

	debug("Creating fresh Biome instance with cache buster:", cacheBuster);
	biomeInstance = await Biome.create({ distribution: Distribution.NODE });
	debug("Biome reloaded successfully");
}

/**
 * Format code using Biome and return the result with IR.
 *
 * @param code - The source code to format
 * @param filePath - Virtual file path for language detection
 * @returns The formatting result including output, IR, and diagnostics
 */
export async function formatWithBiome(
	code: string,
	filePath: string,
): Promise<BiomeResult> {
	const biome = await getBiome();
	const { projectKey } = biome.openProject();

	try {
		biome.applyConfiguration(projectKey, {
			html: {
				formatter: {
					selfCloseVoidElements: "always",
				},
			},
		});

		const result = biome.formatContent(projectKey, code, {
			filePath,
			debug: true,
		});

		return {
			output: result.content,
			ir: result.ir ?? "",
			diagnostics: result.diagnostics.map((d) => ({
				description:
					typeof d === "object" && d !== null && "description" in d
						? String(d.description)
						: String(d),
				severity:
					typeof d === "object" && d !== null && "severity" in d
						? String(d.severity)
						: "error",
			})),
		};
	} catch (err) {
		return {
			output: code,
			ir: "",
			diagnostics: [
				{
					description: err instanceof Error ? err.message : String(err),
					severity: "error",
				},
			],
		};
	}
}
