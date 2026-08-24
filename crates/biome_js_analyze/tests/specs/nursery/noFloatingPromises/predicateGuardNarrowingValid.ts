/* should not generate diagnostics */

type SyncTask = { run: () => void };

function isSyncTask(value: unknown): value is SyncTask {
	return typeof value === "object" && value !== null && "run" in value;
}

// The narrowed method does not return a promise.
function syncCall(value: unknown) {
	if (isSyncTask(value)) {
		value.run();
	}
}

function isNumber(value: Promise<void> | number): value is number {
	return typeof value === "number";
}

// `value` is narrowed to `number`, so the bare statement cannot be a
// floating promise.
function narrowedToNumber(value: Promise<void> | number) {
	if (isNumber(value)) {
		value;
	}
}

declare const rest: unknown[];

function isPromiseAt(first: unknown, value: unknown): value is Promise<void> {
	return value instanceof Promise;
}

// A spread before the checked argument makes the argument-to-parameter
// mapping ambiguous: at runtime `value` may not be the parameter the
// predicate describes. Narrowing must not apply, so `value` keeps its
// declared type and the bare statement does not float.
function spreadBeforeArgument(value: unknown) {
	if (isPromiseAt(...rest, value)) {
		value;
	}
}
