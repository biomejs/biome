/* should not generate diagnostics */

async function returnAny() {
	return "" as any;
}

// See https://github.com/biomejs/biome/issues/11214
function someFn<MyGeneric>();
function someFn<MyGeneric>() {
	const a: MyGeneric = returnAny();
	console.log(a);
}
someFn();

class C {
	m<T>(): void;
	m<T>(x?: T): void {
		console.log(x);
	}

	static s<T>(): void;
	static s<T>(x?: T): void {
		console.log(x);
	}
}
new C().m();
C.s();
