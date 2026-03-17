/* should not generate diagnostics */

// Issue #9519: variables used only in `typeof` type expressions should not be flagged.

// Original reproduction: destructured variable used in typeof type argument
export function MyComponent() {
	const { a } = useThing();
	return someFunction<typeof a>();
}

// Simple variable used only in typeof type annotation
export function example1() {
	const x = getValue();
	type T = typeof x;
	return null as unknown as T;
}

// Variable used in typeof inside a conditional type
export function example2() {
	const config = getConfig();
	type Result = typeof config extends { enabled: true } ? "on" : "off";
	return null as unknown as Result;
}

// Variable used in typeof in a return type annotation
export function example3(): typeof result {
	const result = compute();
	return result;
}

// Variable used in typeof in a parameter type
export function example4(callback: (val: typeof defaults) => void) {
	const defaults = { a: 1, b: 2 };
	callback(defaults);
}

// Qualified name in a type position: namespace-like value access (A.B)
namespace NS {
	export type Inner = number;
}
const ns = NS;
type Q = typeof ns;
export type { Q };

// Variable used only in typeof with nested property access
export function example5() {
	const obj = { nested: { value: 42 } };
	type Deep = typeof obj.nested.value;
	return null as unknown as Deep;
}
