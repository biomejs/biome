const foo = () => {
	const x = 2;
	const y = 1;
	return x + y;
};

function bar() {
	const x = 2;
	const y = 1;
	return x + y;
}

function name() {
	var x = 5;

	var x = 2;
}

function foo(
	aaa = 1,
	bbb = 2,
	ccc = 3
) {
	const x = 4;
	const y = 5;
	return aaa + bbb + ccc + x + y;
}

function parent() {
	var x = 0;
	function nested() {
		var y = 0;
		x = 2;
		var z = x + y;
	}
};

class foo {
	method() {
		let y = 10;
		let x = 20;
		return y + x;
	}
	constructor() {
		let y = 10;
		let x = 20;
		return y + x;
	}
}

(function () {
	let x = 0;
	let y = 0;
	let z = x + y;
	let foo = {};
	return bar;
})();

(() => {
	let x = 0;
	let y = 0;
	let z = x + y;
	let foo = {};
	return bar;
})();