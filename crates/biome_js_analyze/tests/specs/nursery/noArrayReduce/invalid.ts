/* should generate diagnostics */
function f(foo: number[]) {
	return foo.reduce(fn);
}
function g(foo: Int32Array) {
	return foo.reduce(fn);
}
const result = array.reduce<Result>((total, item) => transform(total, item), initialValue);
