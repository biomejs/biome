/* should not generate diagnostics */
import { mutation } from "./_generated/server";

export default mutation({
	args: {},
	handler: async (ctx) => {
		await requireUser(ctx);
	},
});
