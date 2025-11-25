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
function f(a, b, c) {
	return a ? b = 1 : c;
}
function f(a, b) {
	return a && (b = 1);
}

function f(a, b, c) {
	return a = (b = c);
}

function f(a, b, c) {
	return (a = 1, b = 2, c = 3);
}
