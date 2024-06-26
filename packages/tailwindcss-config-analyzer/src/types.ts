export type CandidateRule = Array<
	[
		meta: {
			layer: string;
			sort: { index: bigint };
			options?: { values?: Record<string, unknown> };
		},
		rule: unknown,
	]
>;

export type ConfigVariant = {
	name: string;
	isArbitrary: boolean;
	values: string[];
	hasDash: boolean;
	selectors: unknown;
};

export type TailwindContext = {
	candidateRuleMap: Map<string, CandidateRule>;
	offsets: {
		variantOffsets: Map<string, bigint>;
	};
	getVariants: () => ConfigVariant[];
};
