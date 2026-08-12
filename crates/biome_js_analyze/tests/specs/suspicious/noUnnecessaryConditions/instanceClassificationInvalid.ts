// should generate diagnostics

function alwaysFalsyPromise(p: Promise<void> | undefined) {
	if (!p) {
		// `p` is narrowed to `undefined` here, so this condition is always falsy.
		if (p) {
			p;
		}
	}
}

function alwaysTruthyPromise(p: Promise<void>) {
	// A promise instance is always an object, so this condition is always truthy.
	if (p) {
		p;
	}
}
