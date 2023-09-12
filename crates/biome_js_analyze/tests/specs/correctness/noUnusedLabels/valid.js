A: {
	if (foo()) {
		break A;
	}
	bar();
}

B: for (let i = 0; i < 10; ++i) {
	if (foo()) {
		break B;
	}
	bar();
}

A: break A;

A: {
	foo();
	break A;
	bar();
}

A: if (a) {
	foo();
	if (b) break A;
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) break A;
	bar();
}

A: for (var i = 0; i < 10; ++i) {
	foo();
	if (a) continue A;
	bar();
}

A: {
	B: break B;
	C: for (var i = 0; i < 10; ++i) {
		foo();
		if (a) break A;
		if (c) continue C;
		bar();
	}
}
