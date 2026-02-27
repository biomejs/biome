/* should not generate diagnostics */
let foo;

if (isBar) {
	foo = baz;
} else {
	foo = qux;
}

function quux() {
	if (foo) {
		return bar();
	} else {
		return baz();
	}
}
