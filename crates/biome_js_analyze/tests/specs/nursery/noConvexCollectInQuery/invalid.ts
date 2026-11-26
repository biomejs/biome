/* should generate diagnostics */
async function handler(ctx: QueryCtx) {
	await (ctx.db as DatabaseReader).query("messages").collect();
	await ctx.db!.query("messages").collect();
	await (ctx.db.query("messages") satisfies unknown as Query).collect();
	await ctx.db.query<"messages">("messages").collect();
	await (<DatabaseReader>ctx.db).query("messages").collect();
	await (<Query>ctx.db.query("messages")).collect();
}
