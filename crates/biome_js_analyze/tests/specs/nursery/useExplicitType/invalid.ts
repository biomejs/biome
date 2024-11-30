function test(a: number, b: number) {
	return;
}

function test() {
	return;
}

var fn = function () {
	return 1;
};

var arrowFn = () => "test";

class Test {
	constructor() {}
	get prop() {
		return 1;
	}
	set prop() {}
	method() {
		return;
	}
	arrow = () => "arrow";
	private method() {
		return;
	}
}

const obj = {
	method() {
		return "test";
	},
};

const obj = {
	get method() {
		return "test";
	},
};

const func = (value: number) => ({ type: "X", value }) as any;
const func = (value: number) => ({ type: "X", value }) as Action;

export default () => {};
export default function () {}

// check higher order functions
const arrowFn = () => () => {};
const arrowFn = () => function () {};
const arrowFn = () => {
	return () => {};
};

// does not support detecting a return of a function inside other statements like if, switch, etc.
// we check only the first statment
const arrowFn = (a: number) => {
	if (a === 1) {
		return (): void => {};
	} else {
		return (): number => {
			return a + 2;
		};
	}
};
const arrowFn = (a: number) => {
	switch (a) {
		case 1: {
			return (): void => {};
		}
		case 2: {
			return (): void => {};
		}
		default: {
			return (): void => {};
		}
	}
};

function f() {
	if (x) {
		return 0;
	}
	return (): void => {};
}

function fn() {
	let str = "hey";
	return function (): string {
		return str;
	};
}

const x = { prop: () => {} };
const x = { bar: { prop: () => {} } };

interface Array<Type> {
	pop(): Type | undefined;
	push(...items: Type[]): number;
	method();
}

type MyObject = {
	(input: string);
	propertyName: string;
};

abstract class MyClass {
	public abstract method();
}

abstract class P<T> {
	abstract method(): T;
	abstract get poke();
}

declare namespace myLib {
	function makeGreeting(s: string);
}

declare module "foo" {
	export default function bar();
}
