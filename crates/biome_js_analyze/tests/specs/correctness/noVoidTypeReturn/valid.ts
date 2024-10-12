class A {
	f() {
		return undefined;
	}
}

class B {
	f(): void {
		return;
	}
}

function f(): void {
	return;
}

function g(): void {
	return void 0;
}
