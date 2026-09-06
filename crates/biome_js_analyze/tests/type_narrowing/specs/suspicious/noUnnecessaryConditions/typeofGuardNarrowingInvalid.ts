// should generate diagnostics

function alwaysTruthyAfterGuard(x: number | (() => void)) {
	if (typeof x === "function") {
		if (x) {
			x();
		}
	}
}

// A `typeof` guard can also make the inner condition always falsy.
function alwaysFalsyAfterGuard(x: string | undefined) {
	if (typeof x === "undefined") {
		if (x) {
			x;
		}
	}
}
