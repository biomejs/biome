class A {
	constructor() {}
}

class B {
	constructor() {
		return;
	}
}

class C {
	constructor(x) {
		this.x = x;
	}
}

class D {
	constructor(x) {
		if (x > 0) {
			this.x = x;
			return;
		}
		this.x = 0;
	}
}

class E {
	constructor(x) {
		void (() => {
			return x;
		})();
	}
}