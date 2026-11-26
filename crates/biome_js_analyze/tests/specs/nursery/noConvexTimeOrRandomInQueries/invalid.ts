/* should generate diagnostics */
export const asExpression = query({
	handler: (async (ctx: QueryCtx) => Date.now()) as any,
});

export const satisfiesExpression = query({
	args: {},
	handler: async (ctx: QueryCtx) => (Date as DateConstructor).now(),
} satisfies QueryOptions);

export const nonNullAssertion = query({
	handler: async (ctx: QueryCtx) => Math!.random(),
});

const helper = (<T>() => Date.now()) as () => number;

export const wrappedHelper = query({
	handler: async (ctx: QueryCtx) => helper(),
});
