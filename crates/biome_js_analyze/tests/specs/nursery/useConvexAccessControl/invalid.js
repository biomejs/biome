/* should generate diagnostics */
import { action, mutation, query as convexQuery } from "./_generated/server";

export const send = mutation({
	args: {},
	handler: async (ctx) => {
		await ctx.db.insert("messages", { body: "hello" });
	},
});

export const list = convexQuery({
	args: {},
	handler: async (ctx) => {
		return await ctx.db.query("messages").collect();
	},
});

export const run = action({
	args: {},
	async handler(ctx) {
		await fetch("https://example.com");
	},
});

// The old syntax passes the handler directly.
export const legacy = mutation(async (ctx) => {
	await ctx.db.insert("messages", { body: "hello" });
});

// A check inside an `if` body or a `try` block isn't detected.
export const nested = mutation({
	args: {},
	handler: async (ctx) => {
		if (ctx) {
			await requireUser(ctx);
		}
		try {
			await requireUser(ctx);
		} catch {}
	},
});

// A check inside a nested function isn't detected.
export const nestedFunction = mutation({
	args: {},
	handler: async (ctx) => {
		const verify = async () => {
			await requireUser(ctx);
		};
		await verify();
	},
});

// Names that only start with an access control word don't count.
export const similarNames = mutation({
	args: {},
	handler: async (ctx) => {
		await canister(ctx);
		await requirement(ctx);
		await ctx.has(ctx);
		const mod = require("./auth");
	},
});

// A check passed as an argument isn't a top-level call.
export const argument = mutation({
	args: {},
	handler: async (ctx) => {
		await log(requireUser(ctx));
	},
});

// In an `if` condition, a check nested in another call or a ternary isn't detected.
export const ifConditionCall = mutation({
	args: {},
	handler: async (ctx) => {
		if (log(await canEditNote(ctx))) {
			throw new Error("Unauthorized");
		}
		if (ctx ? await canEditNote(ctx) : false) {
			throw new Error("Unauthorized");
		}
	},
});

const exportedLater = convexQuery({
	args: {},
	handler: async (ctx) => {},
});
export { exportedLater, exportedLater as renamed };

const exportedAsDefault = mutation({
	args: {},
	handler: async (ctx) => {},
});
export default exportedAsDefault;
