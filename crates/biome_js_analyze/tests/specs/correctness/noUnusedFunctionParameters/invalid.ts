class D {
	f(a: D): D | undefined { return; }
}

function withOneUnused({ a, b }): unknown {
	return b
}

function withTwoUnused({ a, b, c }): unknown {
	return b
}
