/*before*/ A /*inner*/: /*after*/ while(true) {};

A: for (let i = 0; i < 10; ++i) {
	foo();
}

A: {
	foo();
}

A: {
	foo();
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) break;
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) continue;
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	H: break A;
}

A: {
	var I = 0;
	console.log(I);
}

A: {
	function f() {
		A: {
			break A;
		}
	}
}

A: {
	class X {
		static {
			B: {
				break B;
			}
		}

		method() {
			B: {
				break B;
			}
		}
	}
}

/*
 * Below is fatal errors.
 * "A: break B",
 * "A: function foo() { break A; }",
 * "A: class Foo { foo() { break A; } }",
 * "A: { A: { break A; } }"
 */

// We are not in a Svelte component
$: {}