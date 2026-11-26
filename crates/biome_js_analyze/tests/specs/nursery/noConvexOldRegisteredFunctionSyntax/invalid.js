/* should generate diagnostics */
export const list = query(async (ctx) => {
	return await ctx.db.query("messages").collect();
});

export const send = mutation(async (ctx, { body }) => {
	await ctx.db.insert("messages", { body });
});

export const run = action(async function (ctx) {});

export const noParameters = internalQuery(() => null);

export const singleParameter = internalMutation(ctx => null);

export const withArgs = internalAction(async (ctx, args) => {});

export let reassignable = query(async (ctx) => {});

export const first = query(async (ctx) => {}),
	second = mutation(async (ctx) => {});

export const commented = query(
	// Lists every message.
	async (ctx) => {}, // trailing
);

export const parenthesizedArgument = query((async (ctx) => {}));

export const parenthesizedCall = (mutation(async (ctx) => {}));

export const parenthesizedComments = query((/* leading */ async (ctx) => {} /* trailing */));
