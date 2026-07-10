/* should generate diagnostics */
function f(a) {
	return (a = 1) as number;
}

function f(a) {
	return (a = 1)!;
}

function f(a) {
	return (a = 1) satisfies number;
}

function f(a) {
	return <number>(a = 1);
}
