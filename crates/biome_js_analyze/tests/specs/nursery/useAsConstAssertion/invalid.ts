// ref: https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/tests/rules/prefer-as-const.test.ts

let foo = { bar: 'baz' as 'baz' };
let foo = { bar: 1 as 1 };
let []: 'bar' = 'bar';
let foo: 'bar' = 'bar';
let foo: 2 = 2;
let foo: 'bar' = 'bar' as 'bar';
let foo = <'bar'>'bar';
let foo = <4>4;
let foo = 'bar' as 'bar';
let foo = 5 as 5;

class foo {
  bar: 'baz' = 'baz';
}

class foo {
  bar: 2 = 2;
}

class foo {
  foo = <'bar'>'bar';
}

class foo {
  foo = 'bar' as 'bar';
}

class foo {
  foo = 5 as 5;
}

class foo {
	constructor(protected prop = 1 as 1) {}
}

function foo(param = 1 as 1) {}

class foo {
	constructor(protected prop = 'bar' as 'bar') {}
}

function foo(param = 'bar' as 'bar') {}
