// should generate diagnostics

function narrowedToPromise(x: Promise<void> | undefined) {
	if (x) {
		// `x` is narrowed to `Promise<void>` here, so the bare statement floats.
		x;
	}
}
