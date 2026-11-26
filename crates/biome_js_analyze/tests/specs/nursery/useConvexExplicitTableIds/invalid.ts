/* should generate diagnostics */
import type { Id } from "./_generated/dataModel";

export async function run(ctx: MutationCtx, messageId: Id<"messages">, storageId: Id<"_storage">) {
	await ctx.db.get(messageId as Id<"messages">);
	await ctx.db.patch(messageId!, { text: "updated" });
	await (ctx.db satisfies DatabaseWriter).delete(messageId);
	await (ctx.db as DatabaseWriter).replace(messageId, { text: "replaced" });
	await ctx.db!.get(messageId);
	await (ctx.db.system as DatabaseReader["system"]).get(storageId);
	await (ctx.db.get as DatabaseReader["get"])(messageId);
}
