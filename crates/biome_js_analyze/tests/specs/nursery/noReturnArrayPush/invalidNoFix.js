/* should generate diagnostics */
function f1() {
	return /* comment */ array.push(value);
}
function f2() {
	foo()
	return [array].push(value);
}
function f3() {
	return function () { return []; }().push(1);
}
function f4() {
	return (/* comment */ array.push(value));
}
function f5() {
	return `${a}`.split("").push(value);
}
