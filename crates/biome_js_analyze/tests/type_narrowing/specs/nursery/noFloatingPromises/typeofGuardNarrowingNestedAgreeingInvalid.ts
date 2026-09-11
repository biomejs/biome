// should generate diagnostics

// The call floats with or without narrowing. Repeating or reversing the same
// `typeof` check must not make narrowing suppress the diagnostic.
function nestedAgreeingGuards(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		if (typeof x === "function") {
			x();
		}
	}
}

// The same holds when the inner guard spells the comparison the other way
// around.
function nestedReversedGuards(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		if ("function" === typeof x) {
			x();
		}
	}
}
