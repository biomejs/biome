/* should generate diagnostics */
export const now = query({
	args: {},
	handler: async (ctx) => Date.now(),
});

export const newDate = query({
	handler(ctx) {
		return new Date();
	},
});

export const newDateWithoutParentheses = query({
	handler: async () => new Date,
});

export const dateCall = internalQuery({
	handler: async () => Date(),
});

export const globalThisDate = query({
	handler: async () => globalThis.Date.now(),
});

export const performanceNow = query({
	handler: async () => performance.now(),
});

export const temporal = query({
	handler: async () => {
		const instant = Temporal.Now.instant();
		const date = Temporal.Now.plainDateISO();
		const dateTime = Temporal.Now.plainDateTimeISO();
		const time = Temporal.Now.plainTimeISO();
		const zoned = Temporal.Now.zonedDateTimeISO();
		return { instant, date, dateTime, time, zoned };
	},
});

export const random = query({
	handler: async () => {
		const a = Math.random();
		const b = crypto.randomUUID();
		const c = crypto.getRandomValues(new Uint8Array(4));
		return { a, b, c };
	},
});

export const oldSyntax = query(async (ctx) => Date.now());

export const optionalAndComputed = query({
	handler: async () => [Date.now?.(), Math["random"]()],
});

export const stringKey = query({
	"handler": async () => Date.now(),
});

function isExpired(session) {
	return session.expiresAt < Date.now();
}

export const helperDeclaration = query({
	handler: async (ctx, args) => {
		const session = await ctx.db.get(args.sessionId);
		return isExpired(session) ? null : session;
	},
});

const pickIndex = (length) => Math.floor(Math.random() * length);

export const helperArrow = query({
	handler: async (ctx) => {
		const entries = await ctx.db.query("entries").collect();
		return entries[pickIndex(entries.length)];
	},
});

function outer() {
	return inner();
}

function inner() {
	return Date.now();
}

export const nestedHelpers = query({
	handler: async () => outer(),
});

function ping() {
	return pong() + Date.now();
}

function pong() {
	return ping();
}

export const recursiveHelpers = query({
	handler: async () => pong(),
});

export const inlineCallback = query({
	handler: async () => [1, 2, 3].map(() => Math.random()),
});

function randomize(value) {
	return value * Math.random();
}

export const namedCallback = query({
	handler: async () => [1, 2, 3].map(randomize),
});

export const immediatelyInvoked = query({
	handler: async () => (() => Date.now())(),
});

export const constructorCallback = query({
	handler: async () => new Promise((resolve) => resolve(Date.now())),
});

async function listMessagesHandler(ctx) {
	return Date.now();
}

export const namedHandler = query(listMessagesHandler);

const getMessagesHandler = async (ctx) => Math.random();

export const handlerReference = query({
	handler: getMessagesHandler,
});

const handler = async (ctx) => Date.now();

export const shorthandHandler = query({ handler });

async function sharedHandler(ctx) {
	return Date.now();
}

export const sharedPublic = query(sharedHandler);
export const sharedInternal = internalQuery({ handler: sharedHandler });
