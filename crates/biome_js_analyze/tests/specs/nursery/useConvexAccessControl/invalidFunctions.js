/* should generate diagnostics */
import { query } from "./_generated/server";

// The whole name must match.
export const partial = query({
	args: {},
	handler: async (ctx) => {
		await getCurrentUser(ctx);
	},
});
