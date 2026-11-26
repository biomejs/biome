/* should not generate diagnostics */
export const run = async (ctx, messageId) => {
	await ctx.db.get(messageId);
	await ctx.db.patch(messageId, { text: "updated" });
};
