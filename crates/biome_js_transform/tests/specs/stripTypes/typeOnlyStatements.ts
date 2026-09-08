interface Foo {
	bar: string; // comment inside
}
type Bar = Foo | null; // comment outside
declare const baz: Bar;
declare function qux(): void;
function overloaded(a: string): void;
function overloaded(a: unknown) {}
