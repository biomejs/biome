/* should not generate diagnostics */
Promise.resolve().catch((err) => {
	throw err;
});
