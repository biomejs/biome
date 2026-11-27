/* should generate diagnostics */
function outer() {
	class Inner {
		method(@decorator(fooSync()) x) {}
	}
}
