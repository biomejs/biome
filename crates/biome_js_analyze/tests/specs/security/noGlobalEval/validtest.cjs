function foo() {
	var eval = "foo";
	window[eval]("foo");
}

function foo() {
	var eval = "foo";
	global[eval]("foo");
}

function foo() {
	var eval = "foo";
	globalThis[eval]("foo");
}

function foo(eval) {
	eval("var a = 0");
}
