/* should not generate diagnostics */

function optionalPromise(p: Promise<void> | undefined) {
	if (p) {
		p;
	}
}
