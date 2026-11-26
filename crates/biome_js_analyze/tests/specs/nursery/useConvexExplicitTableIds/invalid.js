/* should generate diagnostics */
export const run = async (ctx, messageId, storageId) => {
	await ctx.db.get(messageId);
	await ctx.db.patch(messageId, { text: "updated" });
	await ctx.db.replace(messageId, { text: "replaced", author: "Alice" });
	await ctx.db.delete(messageId);
	await ctx.db.system.get(storageId);
	await (ctx.db).get(messageId);
	await (ctx.db.get)(messageId);
	await ctx?.db?.get(messageId);
	await ctx.db.get?.(messageId);
	await this.ctx.db.delete(messageId);
	await ctx.db.get(args.messageId);
};
