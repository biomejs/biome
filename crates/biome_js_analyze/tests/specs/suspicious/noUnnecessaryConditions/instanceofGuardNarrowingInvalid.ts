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
