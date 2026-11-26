/* should not generate diagnostics */
export const listTasks = query({
	args: {},
	handler: async (ctx) => {
		return await ctx.db.query("tasks").collect();
	},
});

export const getTask = query({
	args: { id: v.id("tasks") },
	handler: async (ctx, { id }) => {
		return await ctx.db.get(id);
	},
});

export const shorthandArgs = mutation({ args, handler: async (ctx, { id }) => {} });

export const quotedArgs = action({ "args": {}, handler: async (ctx, { id }) => {} });

export const argsAfterHandler = internalQuery({
	handler: async (ctx, { id }) => {},
	args: { id: v.id("tasks") },
});

// A spread or a computed name might add `args`.
export const spread = internalMutation({ ...shared, handler: async (ctx, { id }) => {} });
export const computed = internalAction({ [key]: {}, handler: async (ctx, { id }) => {} });

// Functions that aren't exported aren't registered with Convex.
const notExported = query({ handler: async (ctx, { id }) => {} });

// Only the functions that register Convex functions are checked.
export const other = otherFunction({ handler: async (ctx, { id }) => {} });
export const member = convex.query({ handler: async (ctx, { id }) => {} });

// The handler is defined elsewhere, so the rule can't tell what it is.
export const referencedHandler = query(handler);

// Calls with more or fewer than one argument aren't Convex functions.
export const noArguments = query();
export const twoArguments = query({ handler: async (ctx, { id }) => {} }, options);

export function notAVariable() {
	return query({ handler: async (ctx, { id }) => {} });
}
