a;
const a = 0;

const b = b;

c;
let c;

let d = d;

e;
var e;

var f = f;

function f(a = b, b = 0) {}

function g(a = a) {}

const instance = new Class1();
class Class1 {}

const instance2 = new Class2();
export default class Class2 {}

function run1() {
	class A extends B {}
	class B {}
}

function run2() {
	class C { static p = B; }
	class B {}
}

function run3() {
	class D { [B.name]() {} }
	class B {}
}

function run4() {
	class E { static { B; } }
	class B {}
}

function run5() {
	class F { [B.name] = 1; }
	class B {}
}

function run6() {
	const o = { [B.name]() {} };
	class B {}
}

function run7() {
	class G { static { x; let x; } }
}

function run8() {
	const H = class extends B {};
	class B {}
}
