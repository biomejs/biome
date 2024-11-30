function foo(...args) {}
let a, b, c, d;
foo(a);
foo(a, b,);
foo(a, b, ...c);
foo(...a, ...b, c, ...d,);
