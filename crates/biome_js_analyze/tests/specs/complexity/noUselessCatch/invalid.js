function foo () {
	f();
	try {
		doSomethingThatMightThrowA();
		doSomethingThatMightThrowB();
	} catch (e) {
		throw e;
	}
	g();
}

for(let i = 0; i < 5; i++) {
	try {
		doSomethingThatMightThrowA();
		if (true) {
			doSomethingThatMightThrowB();
		}
	} catch (e) {
		throw e;
	}
}

try {
	doSomethingThatMightThrow();
} catch (e) {
	throw e;
} finally {
	cleanUp();
}
