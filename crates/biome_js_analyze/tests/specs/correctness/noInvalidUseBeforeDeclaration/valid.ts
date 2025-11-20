/* should not generate diagnostics */

let i: I;
interface I {}

let t: T;
type T = T[] | null;

function f() {
	const enum1Member = E1.A;
}
let e: E1;
enum E1 { A, B = E1.A }

let n = N.X;
namespace N {
    export const X = 0;
}

type X = typeof X; const X = 0;

type Bar = {[BAR]: true;};
const BAR = 'bar';

interface NestedBar {child:  {grandChild: {[FOO]: typeof FOO; enumFoo: EnumFoo}}}
const FOO = 'foo';
enum EnumFoo {BAR = 'bar'}

c;
declare const c: number;

const enum2Member = E2.A;
declare enum E2 { A }

namespace Ns {
	const c = new Class();
}
declare class Class {}

export {}
