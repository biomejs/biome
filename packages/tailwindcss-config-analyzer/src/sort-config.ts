import type { TailwindSpec, UtilitySpec } from "./introspect.js";

type Variant = {
	name: string;
	weight: bigint;
};

export type SortConfig = {
	utilities: Array<{
		layer: string;
		classes: Array<string>;
	}>;
	variants: Array<Variant>;
};

function compareBigInt(a: bigint, b: bigint) {
	if (a < b) return -1;
	if (a > b) return 1;
	return 0;
}

function findDuplicates(array: Array<string>) {
	return array.filter((item, index) => array.indexOf(item) !== index);
}

function logDuplicates(layer: string, array: Array<string>) {
	const duplicates = findDuplicates(array);
	if (duplicates.length > 0) {
		console.log(`Duplicates found in "${layer}" layer: `, duplicates);
	}
}

export function sortConfigFromSpec(
	spec: TailwindSpec,
	{ layerOrder }: { layerOrder: Array<string> },
): SortConfig {
	const utilities = buildConfigUtilities(spec, layerOrder);
	const variants = buildConfigVariants(spec);

	return { utilities, variants };
}

function buildConfigUtilities(spec: TailwindSpec, layerOrder: Array<string>) {
	const utilitiesByLayer = new Map<string, Set<UtilitySpec>>();
	for (const utilitySpec of spec.utilities) {
		const layer = utilitiesByLayer.get(utilitySpec.layer) ?? new Set();
		layer.add(utilitySpec);
		utilitiesByLayer.set(utilitySpec.layer, layer);
	}
	const layerIndexes = new Map(
		layerOrder.map((layer, index) => [layer, index]),
	);
	const utilities = [...utilitiesByLayer.keys()]
		.sort((a, b) => {
			const indexA = layerIndexes.get(a);
			const indexB = layerIndexes.get(b);
			if (indexA == null || indexB == null) return 0;
			return indexA - indexB;
		})
		.map((layer) => {
			const layerUtilities = utilitiesByLayer.get(layer);
			if (!layerUtilities) throw new Error("Unknown layer");
			const classes = [...layerUtilities]
				.sort((a, b) => compareBigInt(a.index, b.index))
				.flatMap(({ utility, hasDefault, hasValues }) => {
					const entries: Array<string> = [];
					if (!hasValues || hasDefault) entries.push(`${utility}$`);
					if (hasValues) entries.push(`${utility}-`);
					return entries;
				});
			// this is to track utilities with the same name and different value types, so
			// that we can figure out how to handle them in the future
			logDuplicates(layer, classes);
			return {
				layer,
				classes: [...new Set(classes)], // remove duplicates
			};
		});
	return utilities;
}

function buildConfigVariants(spec: TailwindSpec): Array<Variant> {
	const variants: Array<Variant> = [...spec.variants]
		.sort((a, b) => compareBigInt(a.weight, b.weight))
		.map((item) => ({ name: item.variant, weight: item.weight }));

	return variants;
}
