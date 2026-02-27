/**
 * Prettier formatting integration using the npm prettier package.
 */

import * as prettier from "prettier";
import * as prettierPluginAstro from "prettier-plugin-astro";
import * as prettierPluginSvelte from "prettier-plugin-svelte";

export interface PrettierResult {
	/** The formatted output */
	output: string;
	/** The formatter IR (doc representation) */
	ir: string;
	/** Error message if formatting failed */
	error?: string;
}

// Type for Prettier's debug API (not officially typed)
interface PrettierDebugApi {
	printToDoc: (code: string, options: prettier.Options) => Promise<unknown>;
	formatDoc: (doc: unknown, options?: prettier.Options) => Promise<string>;
}

/**
 * Get the plugins array for a given parser.
 */
function getPluginsForParser(parser: string): prettier.Plugin[] {
	if (parser === "astro") {
		return [prettierPluginAstro];
	}
	if (parser === "svelte") {
		return [prettierPluginSvelte];
	}
	return [];
}

/**
 * Format code using Prettier and return the result with IR.
 *
 * @param code - The source code to format
 * @param parser - The Prettier parser to use (e.g., "babel", "typescript")
 * @returns The formatting result including output, IR, and any error
 */
export async function formatWithPrettier(
	code: string,
	parser: string,
): Promise<PrettierResult> {
	try {
		const plugins = getPluginsForParser(parser);
		const options: prettier.Options = { parser, plugins };

		// Get formatted output
		const output = await prettier.format(code, options);

		// Get IR using Prettier's debug API
		let ir = "";
		try {
			const debugApi = (prettier as unknown as { __debug: PrettierDebugApi })
				.__debug;
			if (debugApi?.printToDoc && debugApi?.formatDoc) {
				const doc = await debugApi.printToDoc(code, options);
				ir = await debugApi.formatDoc(doc, {});
			}
		} catch {
			// IR extraction is best-effort; don't fail if it doesn't work
			ir = "(IR extraction not available)";
		}

		return { output, ir };
	} catch (err) {
		const errorMessage = err instanceof Error ? err.message : String(err);
		return {
			output: code,
			ir: "",
			error: errorMessage,
		};
	}
}
