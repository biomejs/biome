/* should generate diagnostics */
declare const _foo: number;
const bar_: string = "";
export const _baz = 1;
function _overloaded(a: string): void;
function _overloaded(a: number): void;
function _overloaded(a: unknown) {}
(foo as Foo)._bar;
foo!._bar;
