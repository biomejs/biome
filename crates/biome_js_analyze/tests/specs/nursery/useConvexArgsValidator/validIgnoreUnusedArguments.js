/* should not generate diagnostics */
export const listTasks = query({
	handler: async (ctx) => {
		return await ctx.db.query("tasks").collect();
	},
});

export const emptyPattern = mutation({ handler: async (ctx, {}) => {} });

export const referencedHandler = query({ handler });

export const oldSyntax = query(async (ctx) => {});
