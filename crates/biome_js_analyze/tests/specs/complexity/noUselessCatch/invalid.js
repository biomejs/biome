try {
	doSomethingThatMightThrow();
} catch (e) {
	throw e;
}

try {
	doSomethingThatMightThrow();
} catch (e) {
	throw e;
} finally {
	cleanUp();
}
