/* should not generate diagnostics */
function f(foo: Set<number>) {
	return foo.reduce(fn);
}
function g(foo: Map<string, number>) {
	return foo.reduce(fn);
}
const total = array.reduce<number>((total, item) => total + item, 0);
