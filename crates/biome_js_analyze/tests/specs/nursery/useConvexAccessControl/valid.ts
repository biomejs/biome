/* should not generate diagnostics */
import { mutation, query } from "./_generated/server";

export const nonNull = query({
	args: {},
	handler: async (ctx) => {
		const user = (await requireUser(ctx))!;
		return user;
	},
});

export const asExpression = mutation({
	args: {},
	handler: async (ctx) => {
		return requireUser(ctx) as User;
	},
});

export const ifCondition = mutation({
	args: {},
	handler: async (ctx) => {
		if (!((await canEditNote(ctx)) as boolean)) {
			throw new Error("Unauthorized");
		}
	},
});
