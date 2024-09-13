/* should not generate diagnostics */

/[a-Z]*/.test("foo");

const REGEX = /[a-Z]*/;

function foo(someString) {
	return REGEX.test(someString)
}

const foo = {
	regex: /[a-Z]*/
}

class Foo {
	static regex = /[a-Z]*/;
}

function foo(str) {
	return /[a-Z]*/g.exec(str)
}

function foo(str) {
	return /[a-Z]*/y.exec(str)
}
