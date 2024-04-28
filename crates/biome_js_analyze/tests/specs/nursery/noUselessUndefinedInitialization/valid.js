/* should not generate diagnostics */
const foo = undefined;

const { bar = undefined } = baz;

[quux = undefined] = quuux;

(foo = undefined) => {};

class Foo {
	bar = undefined;
}
