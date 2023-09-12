/* should not generate diagnostics */
var a = a;
let a = a;
const a = a;
a = b;
a += a;
a = +a;
a = [a];
a &= a;
a |= a;
[a, b] = [b, a];
[a = 1] = [a];
[x, a] = [...x, a];
a.c = b.c;
a.b.c = a.Z.c;
a[b] = a[c];
