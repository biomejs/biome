import type { Config } from "tailwindcss";
// @ts-expect-error
import pkg from "tailwindcss/lib/lib/setupContextUtils.js";
import resolveConfig from "tailwindcss/resolveConfig.js";
import type { TailwindContext } from "./types.js";

const { createContext } = pkg;
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

type VariantSpec = {
	variant: string;
	weight: bigint;
};

function introspectVariants(context: TailwindContext): Set<VariantSpec> {
	const variants = new Set<VariantSpec>();
	// This method returns a list of Variants, each with values but offsets are missing.
	const configVariants = context.getVariants();
	// This Map contains weights for each variant name, including those that are values of a main variant.
	const variantOffsets = context.offsets.variantOffsets;

	// TODO: Handle isArbitrary like `has-[]` or `group-has-[]`
	for (const { name, isArbitrary, values } of configVariants) {
		const offset = variantOffsets.get(name);
		if (!offset) continue;

		variants.add({
			variant: name,
			weight: offset,
		});

		for (const value of values) {
			const composedVariantName = `${name}-${value}`;

			const composedVariantOffset = variantOffsets.get(composedVariantName);
			if (!composedVariantOffset) continue;

			variants.add({
				variant: composedVariantName,
				weight: composedVariantOffset,
			});
		}
	}

	return variants;
}

export type TailwindSpec = {
	utilities: Set<UtilitySpec>;
	variants: Set<VariantSpec>;
};

export function introspectTailwindConfig(
	config: Partial<Config>,
	options: { excludedLayers?: Array<string> } = {},
): TailwindSpec {
	const context = createContextFromConfig(config);
	const utilities = introspectUtilities(context, options);
	const variants = introspectVariants(context);
	return { utilities, variants };
}
