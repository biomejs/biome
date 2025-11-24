/* should generate diagnostics */
function f(a) {
	return a = 1;
}

function f(a) {
	return (a = 1);
}

function f(a, b, c) {
	return (a, b, c = 1);
}

function f(a, b, c) {
	return a == (b = c);
}
