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
export type TailwindContext = {
	candidateRuleMap: Map<string, CandidateRule>;
};
