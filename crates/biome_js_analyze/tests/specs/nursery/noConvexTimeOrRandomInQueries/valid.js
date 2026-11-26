/* should not generate diagnostics */
import { getNow } from "./time";

export const createPost = mutation({
	args: { body: v.string() },
	handler: async (ctx, args) => {
		await ctx.db.insert("posts", { body: args.body, publishedAt: Date.now() });
	},
});

export const sendEmail = action({
	handler: async () => crypto.randomUUID(),
});

export const internalWrite = internalMutation({
	handler: async () => Math.random(),
});

export const formatDate = query({
	args: { timestamp: v.number() },
	handler: async (ctx, args) => new Date(args.timestamp).toISOString(),
});

export const otherDateMethods = query({
	handler: async (ctx, args) => [
		Date.parse(args.date),
		Date.UTC(2024, 0, 1),
		Math.floor(args.value),
		Temporal.Now.timeZoneId(),
		crypto.subtle,
	],
});

export const shadowedGlobals = query({
	handler: async (ctx, { Date, Math }) => [Date.now(), Math.random()],
});

export const shadowedGlobalThis = query({
	handler: async (ctx, globalThis) => globalThis.Date.now(),
});

export const uncalledFunction = query({
	handler: async () => {
		const getTime = () => Date.now();
		return getTime;
	},
});

export const functionInObject = query({
	handler: async () => ({
		getTime: () => Date.now(),
		get time() {
			return Date.now();
		},
	}),
});

export const nestedClass = query({
	handler: async () =>
		class {
			method() {
				return Math.random();
			}
		},
});

let reassigned = () => Date.now();
reassigned = () => 0;

export const reassignedHelper = query({
	handler: async () => reassigned(),
});

export const importedHelper = query({
	handler: async () => getNow(),
});

export const twoArguments = query({ handler: async () => Date.now() }, extra);

export const memberCallee = custom.query({
	handler: async () => Date.now(),
});

export const otherRegistrar = queryGeneric({
	handler: async () => Date.now(),
});

function helper() {
	return 1;
}

export const deterministicHelper = query({
	handler: async () => helper(),
});

Date.now();
