/* should generate diagnostics */

// Bare `return;` alongside a value return.
function a(x) {
	if (x) {
		return true;
	}
	return;
}

// Implicit fall-off alongside a value return.
function b(x) {
	if (x) {
		return true;
	}
}

// Arrow function with a block body.
const c = (x) => {
	if (x) {
		return 1;
	}
	return;
};

// Method.
const obj = {
	m(x) {
		if (x) {
			return 1;
		}
	},
};
