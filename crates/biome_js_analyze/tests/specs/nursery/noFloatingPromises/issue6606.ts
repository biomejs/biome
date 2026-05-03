/* should not generate diagnostics */

// Optional chaining on a possibly-undefined function property
type MaybeCallbacks = {
	onSuccess?: () => Promise<void>;
	onError?: () => void;
};
function handleCallbacks(cbs: MaybeCallbacks) {
	cbs.onError?.();
}

// Optional chaining with nullish coalescing — result is not a Promise
const maybeObj: { fetch?: () => string } | undefined = undefined;
maybeObj?.fetch?.() ?? "default";

// Optional chaining on method returning non-Promise
type Api = {
	getName?: () => string;
};
function callApi(api: Api) {
	api.getName?.();
}

// Record with string keys, values are non-Promise functions
const handlers: Record<string, (() => void) | undefined> = {};
handlers?.["someHandler"]?.();

// Nested optional chaining on non-Promise types
type Nested = {
	inner?: {
		doWork?: () => number;
	};
};
function nested(n: Nested) {
	n.inner?.doWork?.();
}

// Optional call on union type that doesn't include Promise
type MaybeFunc = (() => string) | undefined;
const maybeFn: MaybeFunc = undefined;
maybeFn?.();

// Nullish coalescing with optional chaining — result is not a Promise
const config: { getValue?: () => string } | null = null;
const result = config?.getValue?.() ?? "fallback";

// Logical AND with optional chaining — short-circuit, no Promise
const obj: { run?: () => void } | undefined = undefined;
obj && obj.run?.();
