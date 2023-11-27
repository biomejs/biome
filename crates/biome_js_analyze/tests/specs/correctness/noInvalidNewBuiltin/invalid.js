var foo = new Symbol("foo");
function bar() {
	return function Symbol() {};
}
var baz = new Symbol("baz");

var foo = new BigInt(9007199254740991);
function bar() {
	return function BigInt() {};
}
var baz = new BigInt(9007199254740991);
