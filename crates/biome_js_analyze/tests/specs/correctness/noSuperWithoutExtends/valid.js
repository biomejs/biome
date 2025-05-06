/* should not generate diagnostics */
class A {}
class B {
	a = class extends A {
		constructor(_) {
			super();
		}
	};
}
