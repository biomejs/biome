// not the global Math
function case1() {
	let Math;
	Math.pow(a, b);
}

function case2(Math) {
	Math.pow(a, b);
}

var case3 = function Math() {
	Math.pow(a, b);
}

function case4() {
	Math.pow(a, b);
	var Math;
}

function case5() {
	if (foo) {
		const Math = 1;
		Math.pow(a, b);
	}
}
