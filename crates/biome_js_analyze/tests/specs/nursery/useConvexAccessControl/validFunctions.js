/* should not generate diagnostics */
import { query } from "./_generated/server";

export const listed = query({
	args: {},
	handler: async (ctx) => {
		await getCurrentUserOrThrow(ctx);
	},
});

// The default prefixes still apply.
export const defaultPrefix = query({
	args: {},
	handler: async (ctx) => {
		await requireUser(ctx);
	},
});
