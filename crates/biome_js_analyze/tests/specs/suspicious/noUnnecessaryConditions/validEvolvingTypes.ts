/* should not generate diagnostics */
// A `let` initialized with `null` takes the types of the values that the code
// assigns to it, as in TypeScript, so each condition below can change.

declare const enabled: boolean;

function written() {
	let label = null;
	if (enabled) label = "on";
	if (label) {
	}
}

function counted(items: string[]) {
	let count = null;
	for (const item of items) count = item.length;
	return count === null ? "none" : count;
}

// Biome can't type a call to a method whose return type is inferred as
// TypeScript does, so after such a write the variable can hold any value.
const store = {
	compute() {
		return 1;
	},
};
declare const fallback: number;

function computed() {
	let result = null;
	result = store.compute();
	if (result) {
	}
}

function computedWithoutBraces() {
	let result = null;
	if (enabled) result = store.compute();
	if (result) {
	}
}

function computedWithFallback() {
	let result = null;
	result = store.compute();
	return result || fallback;
}
