/* should not generate diagnostics */
// The name of a function expression can be redeclared in its body.
const g = function g() {
	let g;
};

// Distinct names in a body and its parameters.
function k(p) {
	const q = 1;
}

// A nested scope can shadow its parent.
let cd = 1;
{
	let cd = 2;
}

// Mergeable declarations.
function ov(): void;
function ov(a?: number): void {}
interface I {}
interface I {}
namespace I {}

enum E {
	A,
	B,
}

type C<X> = X extends (infer U)[] ? U : X extends Array<infer U> ? U : never;
