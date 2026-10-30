/* should not generate diagnostics */
declare function _declared(): void;
type _Foo = { _bar: string };
interface _Baz {
	_qux: number;
}
let x: Namespace._Type;
function foo(this: Foo, _bar: string) {}
