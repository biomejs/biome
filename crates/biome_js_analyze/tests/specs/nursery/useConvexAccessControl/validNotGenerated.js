/* should not generate diagnostics */
import { mutation, query } from "./functions";

export const send = mutation({
	args: {},
	handler: async (ctx) => {},
});

export const list = query({
	args: {},
	handler: async (ctx) => {},
});
