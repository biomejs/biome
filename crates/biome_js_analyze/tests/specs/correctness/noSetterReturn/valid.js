// early-return
class A {
	set foo(x) {
		if (x) {
			return;
		}
	}
}

// not a setter
class B {
	set(x) {
		return x;
	}
}

class C {
	set(x) {
		void (() => {
		return x;
		})();
	}
}
