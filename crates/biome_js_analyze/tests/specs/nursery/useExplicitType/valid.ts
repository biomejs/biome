/* should not generate diagnostics */
function test(): void {
	return;
}

var fn = function (): number {
	return 1;
};

var arrowFn = (): string => "test";

class Test {
	constructor() {}
	get prop(): number {
		return 1;
	}
	set prop() {}
	method(): void {
		return;
	}
	arrow = (): string => "arrow";
}

var obj = {
	method(): string {
		return "test";
	},
};

var obj = {
	get method(): string {
		return "test";
	},
	set method(val: string) {
		console.log(val);
	},
};

export default (): void => {};
export default function (): void {}

// check direct var assertions
var func = (value: number) => ({ foo: "bar", value }) as const;
var func = () => x as const;

// check allow expressions
node.addEventListener("click", () => {});
node.addEventListener("click", function () {});
fn(() => {});
fn(function () {});
new Promise(() => {});
new Foo(1, () => {});
(function () {
	console.log("This is an IIFE");
})();
(() => {
	console.log("This is an IIFE");
})();
setTimeout(function () {
	console.log("Hello!");
}, 1000);

// check higher order functions
var arrowFn = () => (): void => {};
var arrowFn = () => function (): void {};
var arrowFn = () => {
	return (): void => {};
};

// type assertion
var asTyped = (() => "") as () => string;
var castTyped = <() => string>(() => "");

// variable declarator with a type annotation
type FuncType = () => string;
var arrowFn: FuncType = () => "test";
var funcExpr: FuncType = function () {
	return "test";
};

// default parameter with a type annotation
type CallBack = () => void;
var f = (gotcha: CallBack = () => {}): void => {};
function f(gotcha: CallBack = () => {}): void {}

// class property with a type annotation
type MethodType = () => void;
class App {
	private method: MethodType = () => {};
}

// function as a property or a nested property of a typed object
var x: Foo = { prop: () => {} };
var x = { prop: () => {} } as Foo;
var x = <Foo>{ prop: () => {} };

var x: Foo = { bar: { prop: () => {} } };

class Accumulator {
	private count: number = 0;
	public accumulate(fn: () => number): void {
		this.count += fn();
	}
}
new Accumulator().accumulate(() => 1);

// Returning object from function
interface Behavior {
  namedFunc: () => string;
  arrowFunc: () => string;
}

function getObjectWithFunction(): Behavior {
  return {
    namedFunc: function myFunc(): string { return "value" },
    arrowFunc: () => {},
  }
}

var getObjectWithFunction1 = (): Behavior => {
	return {
		namedFunc: function myFunc(): string { return "value" },
		arrowFunc: () => {},
	}
}


interface Array<Type> {
	pop(): Type | undefined;
	push(...items: Type[]): number;
}

type MyObject = {
	(input: string): string;
	propertyName: string;
};

abstract class MyClass {
	public abstract method(): string;
}

abstract class P<T> {
	abstract method(): T;
	abstract get poke(): string;
}

declare namespace myLib {
	function makeGreeting(s: string): string;
}

declare module "foo" {
	export default function bar(): string;
}

var X: Type = { prop: () => {} };
f({ prop: () => {} })

var foo: unknown[] = arr.map((i) => i * i);
new Promise((resolve) => resolve(1));
new Promise(resolve => resolve(1));

const x: string | null = someFunc();
let x: string | null;
var x: string | null;

const x = "";
const x = 1;
const x = null;
const x = undefined;

let x = "";
let x = 1;

var x = "";
var x = 1;

const fn = (x: number): void => {};

var obj = {
	x: 1,
	func: (x: number): void => {},
	meth(x: number): string {},
}
let obj = {
	x: 1,
	func: (x: number): void => {},
	meth(x: number): string {},
}
const obj = {
	x: 1,
	func: (x: number): void => {},
	meth(x: number): string {},
}

const obj = { dynamic: someFunc() as string }
const obj = { dynamic: <string>(someFunc()) }

namespace Ns {
	export const X = {};
	export function func(arg: string): void {}
}
