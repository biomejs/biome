/* should generate diagnostics */
import { query } from "./_generated/server";

// The option replaces the default prefixes.
export const defaultPrefix = query({
	args: {},
	handler: async (ctx) => {
		await requireUser(ctx);
	},
});

// A prefix must be followed by an uppercase letter or `_`.
export const notAWord = query({
	args: {},
	handler: async (ctx) => {
		await authorized(ctx);
	},
});
