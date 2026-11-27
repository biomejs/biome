/* should generate diagnostics */

async function typeWrappers(ctx: QueryCtx) {
	await (ctx.db as DatabaseReader).query("messages").filter((q) => q.eq(q.field("read"), false)).collect();
	await ctx.db!.query("messages").filter((q) => q.eq(q.field("read"), false)).collect();
	await (ctx.db.query("messages") satisfies object).filter((q) => q.eq(q.field("read"), false)).collect();
}
