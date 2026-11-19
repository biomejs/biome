/* should generate diagnostics */

export const listByAuthor = query({
	handler: async (ctx, args) => {
		return await ctx.db
			.query("messages")
			.filter((q) => q.eq(q.field("author"), args.author))
			.collect();
	},
});

async function chained(ctx) {
	await ctx.db
		.query("messages")
		.withIndex("by_channel", (q) => q.eq("channel", "general"))
		.order("desc")
		.filter((q) => q.eq(q.field("deleted"), false))
		.take(10);
}

async function withDb(db) {
	await db.query("messages").filter((q) => q.eq(q.field("read"), false)).first();
}

async function privateSystem(ctx, db) {
	await db.privateSystem.query("_storage").filter((q) => q.eq(q.field("size"), 0)).collect();
	await ctx.db.privateSystem.query("_storage").filter((q) => q.eq(q.field("size"), 0)).collect();
}

async function optionalChaining(ctx) {
	await ctx?.db?.query("messages")?.filter((q) => q.eq(q.field("read"), false)).collect();
}

async function computedMember(ctx) {
	await ctx["db"].query("messages")["filter"]((q) => q.eq(q.field("read"), false)).collect();
}

async function parenthesized(ctx) {
	await (ctx.db.query("messages")).filter((q) => q.eq(q.field("read"), false)).collect();
}

async function storedInVariable(ctx) {
	const messages = ctx.db.query("messages");
	await messages.filter((q) => q.eq(q.field("read"), false)).collect();
}

async function storedInVariableChain(ctx) {
	const base = ctx.db.query("messages");
	const ordered = base.order("desc");
	await ordered.filter((q) => q.eq(q.field("read"), false)).collect();
}

async function repeatedFilter(ctx) {
	await ctx.db
		.query("messages")
		.filter((q) => q.eq(q.field("read"), false))
		.filter((q) => q.eq(q.field("deleted"), false))
		.collect();
}
