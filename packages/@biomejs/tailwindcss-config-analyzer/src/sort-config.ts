import type { TailwindSpec, UtilitySpec } from "./introspect";

export type SortConfig = {
	utilities: Array<{
		name: string;
		classes: Array<string>;
	}>;
};

function compareBigInt(a: BigInt, b: BigInt) {
	if (a < b) return -1;
	if (a > b) return 1;
	return 0;
}

function findDuplicates(array: Array<string>) {
	return array.filter((item, index) => array.indexOf(item) !== index);
}

function logDuplicates(array: Array<string>) {
	const duplicates = findDuplicates(array);
	if (duplicates.length > 0) {
		console.log("Duplicates found: ", duplicates);
	}
}

export function sortConfigFromSpec(
	spec: TailwindSpec,
	{ layerOrder }: { layerOrder: Array<string> },
): SortConfig {
	const utilitiesByLayer = new Map<string, Set<UtilitySpec>>();
	spec.utilities.forEach((utilitySpec) => {
		const layer = utilitiesByLayer.get(utilitySpec.layer) ?? new Set();
		layer.add(utilitySpec);
		utilitiesByLayer.set(utilitySpec.layer, layer);
	});
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
				.sort((a, b) => {
					const result = compareBigInt(a.index, b.index);
					if (result !== 0) return result;
					return a.utility.localeCompare(b.utility);
				})
				.flatMap(({ utility, hasDefault, hasValues }) => {
					const entries: Array<string> = [];
					if (!hasValues || hasDefault) entries.push(utility);
					if (hasValues) entries.push(`${utility}-`);
					return entries;
				});
			logDuplicates(classes);
			return {
				name: layer,
				classes: [...new Set(classes)], // remove duplicates
			};
		});

	return { utilities };
}
