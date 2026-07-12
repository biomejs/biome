/* should generate diagnostics */

// Type literal with a sole index signature.
type A = { [key: string]: unknown };

// Interface with a sole index signature.
interface B {
	[key: string]: unknown;
}

// Inline annotation.
let c: { [key: number]: string };

// Non-string key type.
type D = { [key: symbol]: () => void };
