// should generate diagnostics

function nestedAgreeingGuards(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		if (typeof x === "function") {
			// Both guards agree on the same tag, so `x` is still narrowed
			// to the promise-returning function here.
			x();
		}
	}
}
