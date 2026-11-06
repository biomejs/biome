/* should generate diagnostics */
x * -1;
-1 * x;
x / -1;
a.b * -1;
foo() * -1;
5 * -1;
-1 * 5;
foo?.bar * -1;
tag`str` * -1;
--x * -1;
x++ * -1;
++x * -1;
-y * -1;
+x * -1;
(a + b) * -1;
-1 * (a + b);
(a ? b : c) * -1;
(a, b) * -1;
(a = b) * -1;
(a + b) / -1;
(x) * -1;
(x * -1) ** 2;
a-x*-1;
a - x * -1;
function f() { return x * -1; }
x * -1 * -1;
a + b * -1;
x * (-1);
x * -(1);
x * -1.0;
x * -0x1;
(() => 1) * -1;
async function g() { return await x * -1; }
x /* c */ * -1;
x * -1 /* c */;
/* leading */ (a + b) * -1;
-1 * x // trailing
do {} while (ready)
x * -1;
