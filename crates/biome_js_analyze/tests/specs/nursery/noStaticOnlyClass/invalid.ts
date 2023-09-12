class X0 {
	static foo = false;
	static bar() { }
}

class X1 {
	static #foo = false;
	static bar() {}
}

const X2 = class {
	static foo = false;
};

const X3 = class A {
	static foo = false;
};

export default class {
	static foo = false;
}

class StaticConstants0 {
	static readonly version = 42;

	static isProduction() {
		return Math.random() > 0.5;
	}
}
