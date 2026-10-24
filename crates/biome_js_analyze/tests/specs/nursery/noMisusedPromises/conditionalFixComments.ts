/* should generate diagnostics */
async function f() {
	const p = Promise.resolve(1);
	if (
		// a leading comment
		p
	) {
	}
}
