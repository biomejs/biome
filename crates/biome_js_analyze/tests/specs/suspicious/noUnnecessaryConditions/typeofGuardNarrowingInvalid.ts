// should generate diagnostics

function alwaysTruthyAfterGuard(x: number | (() => void)) {
	if (typeof x === "function") {
		// `x` is narrowed to a function here, so this condition is always truthy.
		if (x) {
			x();
		}
	}
}
