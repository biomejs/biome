/* should not generate diagnostics */
function foo() {
	throw new Error();
}

const bar = () => new Error();

[new Error()];

const error = new Error();

reject(new Error());

new Foo();

new errorFactory();

new Errors();

new lib.create();

Error();

new Error().stack;
