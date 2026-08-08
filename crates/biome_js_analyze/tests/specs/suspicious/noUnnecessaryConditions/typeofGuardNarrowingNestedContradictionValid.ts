/* should not generate diagnostics */

// `x` cannot be both a string and a function, so this code is unreachable.
// Honouring only the innermost `typeof` would wrongly narrow `x` to a
// function and report `if (x)` as always truthy.
function nestedContradictoryGuards(x: string | (() => void)) {
	if (typeof x === "string") {
		if (typeof x === "function") {
			if (x) {
				x();
			}
		}
	}
}
