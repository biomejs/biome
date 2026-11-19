// should not generate diagnostics


try {
	doSomething();
} catch {
	throw new TypeError("Something went wrong");
}


try {
    doSomething();
} catch (e) {
    console.error(e);
}
