/* should not generate diagnostics */

// `x` is narrowed to `number` inside the guard, so the bare statement cannot
// be a floating promise. Without narrowing this is a false positive.
function narrowedToNumber(x: number | Promise<void>) {
	if (typeof x === "number") {
		x;
	}
}

// The promise is awaited inside the guarded branch.
async function handled(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		await x();
	}
}

// The narrowed callable does not return a promise.
function syncCallable(x: number | (() => void)) {
	if (typeof x === "function") {
		x();
	}
}

// Narrowing `unknown` must not invent a promise-returning type.
function fromUnknown(x: unknown) {
	if (typeof x === "function") {
		x();
	}
}
