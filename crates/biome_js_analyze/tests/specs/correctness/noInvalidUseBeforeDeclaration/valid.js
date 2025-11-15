/* should not generate diagnostics */

g();
function g() { f(); }
function f() {}

export { X }; const X = 1;

let a; console.log(a);

function h() { Y; }; const Y = 0;

function useClassInFunction() {
	const instance = new Class();
}
class Class {
	static SINGLETON = new Class();
}
