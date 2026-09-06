// should generate diagnostics

class Foo {
	a = 1;
}

function alwaysTruthyAfterGuard(x: number | Foo) {
	if (x instanceof Foo) {
		// `x` is narrowed to a `Foo` instance here, so this condition is
		// always truthy.
		if (x) {
			x.a;
		}
	}
}

// The narrowing carries into the left operand of a logical expression too.
function alwaysTruthyInLogicalAnd(x: number | Foo) {
	if (x instanceof Foo) {
		x && x.a;
	}
}
