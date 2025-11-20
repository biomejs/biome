
/* should not generate diagnostics */

class Foo {
	#usedOnlyInWriteStatement = 5;
	method() {
		this.#usedOnlyInWriteStatement += 42;
	}
}
