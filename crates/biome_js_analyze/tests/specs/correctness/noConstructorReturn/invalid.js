class A {
	constructor() {
		return 0;
	}
}

class B {
	constructor() {
		return this;
	}
}

class C {
	constructor(x) {
		this.x = x;
		return x;
	}
}

class D {
	constructor(x) {
		if (x > 0) {
			this.x = x;
			return x;
		}
		this.x = 0;
	}
}