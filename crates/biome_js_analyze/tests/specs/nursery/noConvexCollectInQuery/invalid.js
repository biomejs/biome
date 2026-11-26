/* should generate diagnostics */
async function handler(ctx, db) {
	await ctx.db.query("messages").collect();
	await db.query("messages").collect();
	await ctx.db
		.query("messages")
		.withIndex("by_channel", (q) => q.eq("channel", "general"))
		.order("desc")
		.collect();
	await ctx.db.query("messages").filter((q) => q.eq(q.field("done"), true)).collect();
	await ctx.db?.query("messages").collect();
	await ctx.db.query("messages")?.collect();
	await (ctx.db.query("messages")).collect();
	await (await ctx.db.query("messages")).collect();
	await this.db.query("messages").collect();
	await db.privateSystem.query("_storage").collect();
	await ctx.db.privateSystem.query("_storage").collect();
	await ctx.db.query("messages").fullTableScan().collect();
	await ctx.db.query("messages").withSearchIndex("search", (q) => q.search("body", "hi")).collect();
}
