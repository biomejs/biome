/** should generate diagnostics */

export function run() {
	// `extends` is evaluated while the class definition runs.
	class A extends B {}

	// A computed member name is evaluated while the class definition runs.
	class C {
		[B.name]() {}
	}

	// A static field initializer is evaluated while the class definition runs.
	class D {
		static p = B;
	}

	// A static block is evaluated while the class definition runs.
	class E {
		static {
			B;
		}
	}

	const e = new B();
	class B {}
	return [A, C, D, E, e];
}