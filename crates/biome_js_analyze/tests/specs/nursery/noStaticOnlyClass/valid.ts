const X0 = {
	foo: false,
	bar() {}
};

class X1 {
	static foo = false;
	static bar() {}

	constructor() {}
}

class X2 {
	static foo = false;
	static bar() {}

	unicorn() {}
}

class X3 {
	unicorn() {}
}


class X4 {
	constructor() {}
}

class HelloWorldLogger {
	constructor() {
		console.log('Hello, world!');
	}
}

export const version = 42;

export function isProduction() {
	return Math.random() > 0.5;
}

function logHelloWorld() {
	console.log('Hello, world!');
}

const X5 = class {
	foo = false;
};

export default class {
	foo = false;
}

function sealed(ctor, _ctx) {
	Object.seal(ctor);
	Object.seal(ctor.prototype);
}

// A decorator should prevent the class from being seen as "static-only"
@sealed
class StaticConstants1 {
	static readonly version = 42;

	static isProduction() {
		return Math.random() > 0.5;
	}
}

class Empty {}
