// should generate diagnostics

// Narrowing must keep the promise, so the diagnostic must survive.
function narrowedToPromise(x: Promise<void> | undefined) {
	if (x) {
		x;
	}
}
