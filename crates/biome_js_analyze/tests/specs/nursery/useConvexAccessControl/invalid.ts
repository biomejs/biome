/* should generate diagnostics */
import { mutation } from "./_generated/server";

export const send = mutation({
	args: {},
	handler: (async (ctx) => {
		await ctx.db.insert("messages", { body: "hello" });
	}) satisfies Handler,
}) as RegisteredMutation;

export const expressionBody = mutation({
	args: {},
	handler: async (ctx) => ctx.db.insert("messages", { body: "hello" }),
});
