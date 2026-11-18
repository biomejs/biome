// should not generate diagnostics
// A `let` initialized with `null` takes the type of the value that the code
// assigns to it, as in TypeScript, so the left operand of the `||` below
// can't be nullish.

function assigned(input: string) {
	let label = null;
	label = input;
	return label || "none";
}

// Biome can't type a call to a method whose return type is inferred as
// TypeScript does, so after such a write the variable can hold `0`, and
// checking it only for `null` and `undefined` would change the result.
const store = {
	compute() {
		return 1;
	},
};
declare const fallback: number;

function computed() {
	let result = null;
	result = store.compute();
	return result || fallback;
}

function computedOrReplaced() {
	let result = null;
	result = store.compute();
	if (!result) result = fallback;
	return result;
}
