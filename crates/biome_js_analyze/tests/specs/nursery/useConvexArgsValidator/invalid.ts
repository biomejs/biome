/* should generate diagnostics */
export const typedArguments = query({
	handler: async (ctx, { id }: { id: Id<"tasks"> }) => {
		return await ctx.db.get(id);
	},
});

export const asserted = query({
	handler: async (ctx) => {},
} as const);

export const wrappedCall = query({
	handler: async (ctx, args) => {},
}) satisfies unknown;

export const wrappedHandler = query({
	handler: (async (ctx) => {}) as Handler,
});

export const thisParameter = query({
	handler: async function (this: unknown, ctx) {},
});

export const thisParameterWithArguments = query({
	handler: async function (this: unknown, ctx, args) {},
});
