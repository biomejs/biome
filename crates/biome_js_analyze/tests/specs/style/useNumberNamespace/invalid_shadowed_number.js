/* should generate diagnostics */
function f(Number) {
	return { [NaN]: 1 }.NaN;
}

function g() {
	const Number = null;
	return parseInt("1");
}

class Number {}
const foo = -Infinity;

function h() {
	return globalThis.parseFloat("1.1");
}
