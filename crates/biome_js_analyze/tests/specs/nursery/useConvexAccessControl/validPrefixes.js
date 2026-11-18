/* should not generate diagnostics */
import { query } from "./_generated/server";

export const prefixed = query({
	args: {},
	handler: async (ctx) => {
		await authorizeUser(ctx);
	},
});

export const alone = query({
	args: {},
	handler: async (ctx) => {
		await auth.authorize(ctx);
	},
});

export const underscore = query({
	args: {},
	handler: async (ctx) => {
		await authorize_user(ctx);
	},
});

export const mixedCasePrefix = query({
	args: {},
	handler: async (ctx) => {
		if (!(await isAllowedTo(ctx, "read"))) {
			throw new Error("Unauthorized");
		}
	},
});
