var foo = Symbol("foo");
function bar(Symbol) {
	var baz = new Symbol("baz");
}
function Symbol() {}
new Symbol();
new foo(Symbol);
new foo(bar, Symbol);

var foo = BigInt(9007199254740991);
function bar(BigInt) {
	var baz = new BigInt(9007199254740991);
}
function BigInt() {}
new BigInt();
new foo(BigInt);
new foo(bar, BigInt);
