/* should not generate diagnostics */
function overload(a: string): void;
function overload(a: number): void;
function overload(a: string | number) { a; b; }

declare function declared(): void;

abstract class C {
	abstract method(): void;
}

namespace N { a; b; c; }
