/* should not generate diagnostics */

// Honouring only the innermost `typeof` would narrow `x` to a function and
// report `if (x)` as always truthy. The guards contradict each other, so the
// branch is unreachable rather than `x` being conclusively a function.
function nestedContradictoryGuards(x: string | (() => void)) {
	if (typeof x === "string") {
		if (typeof x === "function") {
			if (x) {
				x();
			}
		}
	}
}

// The same holds when the inner guard states the contradiction with `!==`.
function nestedNegatedContradiction(x: string | (() => void)) {
	if (typeof x === "string") {
		if (typeof x !== "string") {
			if (x) {
				x();
			}
		}
	}
}
