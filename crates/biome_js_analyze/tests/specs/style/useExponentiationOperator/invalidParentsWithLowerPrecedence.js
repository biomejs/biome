// parents with a lower precedence
a * Math.pow(b, c);
Math.pow(a, b) * c;
a + Math.pow(b, c);
Math.pow(a, b)/c;
a < Math.pow(b, c);
Math.pow(a, b) > c;
a === Math.pow(b, c);
a ? Math.pow(b, c) : d;
a = Math.pow(b, c);
a += Math.pow(b, c);
function *f() { yield Math.pow(a, b) }
a, Math.pow(b, c), d
