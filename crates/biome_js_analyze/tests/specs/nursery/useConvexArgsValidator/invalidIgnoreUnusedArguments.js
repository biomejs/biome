/* should generate diagnostics */
export const getTask = query({
	handler: async (ctx, { id }) => {
		return await ctx.db.get(id);
	},
});

export const oldSyntax = mutation(async (ctx, args) => {});

export const onlyRestParameter = query({ handler: async (...params) => {} });
