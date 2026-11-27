/* should generate diagnostics */
function f() {
	interface Number {
		foo(): void;
	}
	return NaN;
}

function g() {
	type Number = string;
	return parseInt("1");
}

function h() {
	enum Number {
		A,
	}
	return parseFloat("1.1");
}
