/* should not generate diagnostics */

// Already a Record.
type A = Record<string, unknown>;

// Index signature alongside another member.
interface B {
	[key: string]: unknown;
	length: number;
}

// readonly index signature is left for a later iteration.
type C = { readonly [key: string]: unknown };

// A plain property type, not an index signature.
type D = { name: string };

// Empty type literal.
type E = {};
