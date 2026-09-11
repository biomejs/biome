// should generate diagnostics

function alwaysTruthyPromise(p: Promise<void>) {
	// A promise instance is always an object, so this condition is always truthy.
	if (p) {
		p;
	}
}
