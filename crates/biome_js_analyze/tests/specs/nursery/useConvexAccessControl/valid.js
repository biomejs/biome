/* should not generate diagnostics */
import {
	action,
	internalAction,
	internalMutation,
	internalQuery,
	mutation,
	query,
} from "./_generated/server";
import { query as solidQuery } from "@solidjs/router";

export const declaration = query({
	args: {},
	handler: async (ctx) => {
		const user = await requireUser(ctx);
		return user;
	},
});

export const expressionStatement = mutation({
	args: {},
	handler: async (ctx) => {
		await ensureAdmin(ctx);
	},
});

export const withoutAwait = mutation({
	args: {},
	handler: async (ctx) => {
		assertSignedIn(ctx);
	},
});

export const returned = query({
	args: {},
	handler: async (ctx) => {
		return await checkAccess(ctx);
	},
});

export const ifCondition = mutation({
	args: {},
	handler: async (ctx) => {
		if (!(await canEditNote(ctx)) || isBanned) {
			throw new Error("Unauthorized");
		}
	},
});

export const ifConditionBinary = mutation({
	args: {},
	handler: async (ctx) => {
		if ((await hasRole(ctx, "admin")) === false) {
			throw new Error("Unauthorized");
		}
	},
});

export const method = mutation({
	args: {},
	handler: async (ctx) => {
		await authz.require(ctx, "documents:delete");
	},
});

export const underscore = mutation({
	args: {},
	handler: async (ctx) => {
		await has_role(ctx, "admin");
	},
});

export const methodHandler = action({
	args: {},
	async handler(ctx) {
		await requireUser(ctx);
	},
});

export const expressionBody = query({
	args: {},
	handler: (ctx) => requireUser(ctx),
});

export const legacy = mutation(async (ctx) => {
	await requireUser(ctx);
});

// Internal functions can't be called from the internet.
export const cleanup = internalMutation({
	args: {},
	handler: async (ctx) => {},
});
export const internalRead = internalQuery({ args: {}, handler: async (ctx) => {} });
export const internalRun = internalAction({ args: {}, handler: async (ctx) => {} });

// Functions that aren't exported can't be called from the internet.
const notExported = mutation({
	args: {},
	handler: async (ctx) => {},
});

// A handler that isn't written inline can't be checked.
export const sharedHandler = mutation({
	args: {},
	handler: someHandler,
});

// Only `query`, `mutation` and `action` from `_generated/server` are checked.
export const solid = solidQuery(async () => {});
export const custom = customMutation({
	args: {},
	handler: async (ctx) => {},
});
export const reExported = mutation;
export { other } from "./other";
