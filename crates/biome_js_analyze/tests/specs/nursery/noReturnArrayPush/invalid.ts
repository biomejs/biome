/* should generate diagnostics */
const f1 = (value: string) => array.push(value);
function f2() {
	return array.push(value) as number;
}
function f3() {
	return <number>array.unshift(value);
}
function f4() {
	return array.push(value)!;
}
function f5() {
	return array.unshift(value) satisfies number;
}
const f6 = (value: string) => array.push(value) satisfies number;
function f7() {
	return getArray()!.push(value);
}
