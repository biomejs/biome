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

var foo = new Math();
function bar() {
	return function Math() {};
}
var baz = new Math();

var foo = new JSON();
function bar() {
	return function JSON() {};
}
var baz = new JSON();

var foo = new Reflect();
function bar() {
	return function Reflect() {};
}
var baz = new Reflect();

var foo = new Atomics();
function bar() {
	return function Atomics() {};
}
var baz = new Atomics();

var foo = new Intl();
function bar() {
	return function Intl() {};
}
var baz = new Intl();
