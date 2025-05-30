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
	return aaa + bbb + ccc
}

function parent() {
	var x = 0;
	function nested() {
		var y = 0;
		x = 2;
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
