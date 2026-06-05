/* should not generate diagnostics */
var a=3; var b = (x) => { a++; return x + a; }; setTimeout(() => { b(a); }, 0);
