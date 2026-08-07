// should generate diagnostics

// Repeating the same `typeof` check keeps `x` narrowed to the
// promise-returning function rather than resetting to its declared type.
function nestedAgreeingGuards(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		if (typeof x === "function") {
			x();
		}
	}
}
