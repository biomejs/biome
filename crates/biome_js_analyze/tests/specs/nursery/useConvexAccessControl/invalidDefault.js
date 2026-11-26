/* should generate diagnostics */
import { mutation } from "../convex/_generated/server.js";

export default mutation({
	args: {},
	handler: function (ctx) {
		ctx.db.insert("messages", { body: "hello" });
	},
});
