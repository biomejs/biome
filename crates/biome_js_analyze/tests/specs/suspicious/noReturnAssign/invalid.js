/* should generate diagnostics */

const f = (a) => a = 1;

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
function f(a, b) {
	return a && (b = 1);
}

function f(a, b, c) {
	return a = (b = c);
}

function f(a, b, c) {
	return (a = 1, b = 2, c = 3);
}

function f(a) {
	return [a = 1];
}

async function f(a) {
	return await (a = 1);
}

function f(a) {
	return 5 + (a = 1);
}

function f(a) {
	return (a = 1) + 5;
}

function f(a) {
	return foo(a = 1);
}

function f(a, b) {
	return b[a = 1];
}

function f(a) {
	return (a = 1) ? true : false;
}

function f(a) {
	return true ? a = 1 : false;
}

function f(a, b) {
	return true ? false : a = 1;
}

function f(a, b) {
	return (a = 1) in b;
}

function f(a, Class) {
	return (a = 1) instanceof Class;
}

function f(a, b) {
	return (a = 1) || b;
}

function f(a, b) {
	return a || (b = 1);
}

function f(a, b) {
	return (a = 1) ?? b;
}

function f(a, b) {
	return a ?? (b = 1);
}

function f(a) {
	return !(a = 1);
}

function f(a) {
	return typeof (a = 1);
}

function f(a) {
	return void (a = 1);
}

function f(a) {
	return -(a = 1);
}

function f(a) {
	return <div prop={a = 1} />;
}
