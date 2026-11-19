/* should generate diagnostics */
async function f() {
	// a leading comment
	Promise.resolve(1); // a trailing comment
	Promise.resolve(1) /* a trailing comment */;
}
