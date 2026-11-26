/* should generate diagnostics */
async function f() {
	// a leading comment
	[1, 2].map((x) => Promise.resolve(x));
	[1, 2].map((x) => Promise.resolve(x)) /* a trailing comment */;
}
