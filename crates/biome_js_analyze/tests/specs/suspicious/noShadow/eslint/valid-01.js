/* should not generate diagnostics */

var a=3; function b(x) { a++; return x + a; }; setTimeout(function() { b(a); }, 0);
