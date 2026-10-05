/* should not generate diagnostics */
var a = test[__iterator__];
var __iterator__ = null;
foo[`__iterator__${bar}`];
foo[tag`__iterator__`];
foo["__iterator"];
foo.iterator;
foo[Symbol.iterator] = function* () {};
var obj = { __iterator__: function () {} };
class Foo {
	__iterator__() {}
	#__iterator__ = 1;
	bar() {
		return this.#__iterator__;
	}
}
const { __iterator__ } = foo;
