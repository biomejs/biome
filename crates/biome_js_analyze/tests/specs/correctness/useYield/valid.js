// Valid
function foo() {
	return 0;
}

function* foo() {
	yield 0;
}

function* foo() {}

(function* foo() {
	yield 0;
})();

(function* foo() {})();

const obj = {
	*foo() {
		yield 0;
	},
};

const obj = { *foo() {} };

class A {
	*foo() {
		yield 0;
	}
}

class A {
	*foo() {}
}