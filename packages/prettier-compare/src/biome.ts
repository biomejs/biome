/**
 * Biome formatting integration using @biomejs/js-api
 */

import { Biome, Distribution } from "@biomejs/js-api";

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

let biomeInstance: Biome | null = null;

/**
 * Get or create a Biome instance.
 */
async function getBiome(): Promise<Biome> {
	if (!biomeInstance) {
		biomeInstance = await Biome.create({ distribution: Distribution.NODE });
	}
	return biomeInstance;
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
			formatter: {
				indentStyle: "space",
				indentWidth: 2,
			},
			html: {
				experimentalFullSupportEnabled: true,
				formatter: {
					enabled: true,
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
