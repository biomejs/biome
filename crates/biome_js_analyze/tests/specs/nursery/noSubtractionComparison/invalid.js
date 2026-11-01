/* should generate diagnostics */
if (a - b > 0) {}
if (a - b >= 0) {}
if (a - b < 0) {}
if (a - b <= 0) {}
a - b === 0;
a - b !== 0;
a - b == 0;
a - b != 0;
0 < a - b;
0 <= a - b;
0 > a - b;
0 >= a - b;
0 === a - b;
0 !== a - b;
a - b > 0.0;
a - b > 0x0;
a.length - b.length - c > 0;
(foo.length) - (bar.length) > 0;
(a - b) > 0;
a - b > (0);
((a - b)) > ((0));
a?.b - c?.d > 0;
x = a - b > 0 ? 1 : 2;
