/* should not generate diagnostics */

function optionalPromise(p: Promise<void> | undefined) {
	if (p) {
		p;
	}
}

function genericValue<T>(g: T | undefined) {
	if (!g) {
		// `g` may be a falsy value of `T`, so this condition stays necessary.
		if (g) {
			g;
		}
	}
}
