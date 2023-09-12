/*before*/ A /*inner*/: /*after*/ var foo = 0;

B: {
	foo();
}

C: for (let i = 0; i < 10; ++i) {
	foo();
}

D: var foo = 0;

E: {
	foo();
	bar();
}

F: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) break;
	bar();
}

G: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) continue;
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	H: break A;
}

I: {
	var I = 0;
	console.log(I);
}

J: /* comment */ foo;

K /* comment */: foo;

L: {
	function f() {
		L: {
			break L;
		}
	}
}

M: {
	class X {
		static {
			M: {
				break M;
			}
		}

		method() {
			M: {
				break M;
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
