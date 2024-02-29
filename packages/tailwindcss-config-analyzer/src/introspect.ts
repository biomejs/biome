import type { Config } from "tailwindcss";
import { createContext } from "tailwindcss/lib/lib/setupContextUtils";
import resolveConfig from "tailwindcss/resolveConfig.js";
import type { TailwindContext } from "./types.js";

const DEFAULT_CONFIG: Config = { content: [] };

function createContextFromConfig(config: Partial<Config>) {
	return createContext(resolveConfig({ ...DEFAULT_CONFIG, ...config }));
}

export type UtilitySpec = {
	utility: string;
	layer: string;
	index: bigint;
	hasValues: boolean;
	hasDefault: boolean;
	valueType?: string; // TODO: find a way to disambiguate value types
};

function introspectUtilities(
	context: TailwindContext,
	{ excludedLayers }: { excludedLayers?: Array<string> } = {},
): Set<UtilitySpec> {
	const utilities = new Set<UtilitySpec>();
	for (const [utility, candidates] of context.candidateRuleMap.entries()) {
		for (const [
			{
				layer,
				sort: { index },
				options: { values } = { values: undefined },
			},
			rule,
		] of candidates) {
			if (excludedLayers?.includes(layer)) continue;
			const hasValues = values != null || typeof rule === "function";
			const hasDefault = values != null && "DEFAULT" in values;
			utilities.add({ utility, layer, index, hasValues, hasDefault });
		}
	}
	return utilities;
}

export type TailwindSpec = {
	utilities: Set<UtilitySpec>;
};

export function introspectTailwindConfig(
	config: Partial<Config>,
	options: { excludedLayers?: Array<string> } = {},
): TailwindSpec {
	const context = createContextFromConfig(config);
	const utilities = introspectUtilities(context, options);
	return { utilities };
}
