/* should not generate diagnostics */

const bar = () => {
	const x = 2 + 1;
	return x;
};

function name() {
	var x = 5;
	var x = 2;
}

class foo {
	constructor() {
		let y = 10;
		let x = 20;
		return y + x;
	}
	method() {
		let y = 10;
		let x = 20;
		return y + x;
	}
}

function baz(
	aaa = 1,
	bbb = 2,
	ccc = 3
) {
	const x = 4;
	return aaa + bbb + ccc + x;
}

(function () {})();

(function () {
	let x = 0;
	return bar;
})();
