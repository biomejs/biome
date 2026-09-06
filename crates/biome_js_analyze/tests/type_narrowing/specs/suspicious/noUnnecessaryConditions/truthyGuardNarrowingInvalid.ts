// should generate diagnostics

function alwaysTruthyAfterGuard(x: "on" | null) {
	if (x) {
		// `x` is narrowed to `"on"` here, so this condition is always truthy.
		if (x) {
			x;
		}
	}
}

function alwaysFalsyAfterNegatedGuard(x: "on" | null) {
	if (!x) {
		// `x` is narrowed to `null` here, so this condition is always falsy.
		if (x) {
			x;
		}
	}
}

function alwaysTruthyObjectAfterGuard(x: { a: number } | null) {
	if (x) {
		// `x` is narrowed to an object here, so this condition is always truthy.
		if (x) {
			x.a;
		}
	}
}
