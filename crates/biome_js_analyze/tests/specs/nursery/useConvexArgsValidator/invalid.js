/* should generate diagnostics */
export const listTasks = query({
	handler: async (ctx) => {
		return await ctx.db.query("tasks").collect();
	},
});

export const getTask = query({
	handler: async (ctx, { id }) => {
		return await ctx.db.get(id);
	},
});

export const updateTask = mutation({ handler: async (ctx, args) => {} });

export const sendEmail = action({ handler: async function (ctx) {} });

export const methodHandler = internalQuery({
	async handler(ctx, args) {},
});

export const emptyPattern = internalMutation({
	handler: async (ctx, {}) => {},
});

export const restParameter = internalAction({
	handler: async (ctx, ...rest) => {},
});

export const singleParameter = query({
	handler: async ctx => {},
});

export const referencedHandler = query({ handler });

export const noHandler = query({});

export const withComment = query({
	// Lists every task.
	handler: async (ctx) => {},
});

export const oldSyntax = query(async (ctx) => {});

export const oldSyntaxWithArguments = mutation(async (ctx, { id }) => {});

export const oldSyntaxFunction = action(async function (ctx) {});

export const parenthesized = query(({ handler: async (ctx) => {} }));

export const first = query({ handler: async (ctx) => {} }),
	second = mutation({ handler: async (ctx, args) => {} });

export let reassignable = query({ handler: async (ctx) => {} });

export default query({ handler: async (ctx) => {} });

export const onlyRestParameter = query({ handler: async (...params) => {} });

export const oldSyntaxWithComment = query(
	// Lists every task.
	async (ctx) => {}, // The handler.
);
