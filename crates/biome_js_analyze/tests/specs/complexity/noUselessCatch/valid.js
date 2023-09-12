/* should not generate diagnostics */

try {
	doSomethingThatMightThrow();
} catch (e) {
	doSomethingBeforeRethrow();
	throw e;
}
