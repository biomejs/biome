
g();
function g() { f(); }
function f() {}

new C(); class C {}

export { X }; const X = 1;

let a; console.log(a);

function h() { X; }; const X = 0;
