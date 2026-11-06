/* should generate diagnostics */
function overload(a: string): void;
function overload(a: number): void;
function overload(a: string | number) { a; b; c; }

abstract class C {
	method(): void;
	method() { a; b; c; }
	@decorator
	decorated() { a; b; c; }
}

class D {
	@decorator()
	@other
	public static decorated() { a; b; c; }
}
