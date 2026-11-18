/* should generate diagnostics */
// Comparisons that can never both be true
status_code <= 400 && status_code > 500;
status_code > 500 && status_code < 400;
status_code < 500 && status_code > 500;
status_code <= 500 && status_code > 500;
500 >= status_code && 600 < status_code;
500 >= status_code && status_code > 600;
(status_code > 500) && (status_code < 400);
x > -1 && x < -5;
this.count > 10 && this.count < 5;
a.b.c > 10 && a["b"].c < 5;

// Comparisons with no effect
status_code < 200 && status_code <= 299;
status_code > 200 && status_code >= 299;
status_code >= 500 && status_code > 500;
status_code > 500 && status_code >= 500;
status_code <= 500 && status_code < 500;
status_code < 500 && status_code <= 500;
x < 5 && 5 > x;

// Chains of `&&`
status_code <= 400 && foo() && status_code > 500;
status_code > 500 && foo() && bar && status_code < 400;
foo() && bar && status_code < 500 && status_code > 500;
500 >= status_code && baz && 600 < status_code;
que && 500 >= status_code && baz && status_code > 600;
foo() && status_code >= 500 && status_code > 500;
status_code <= 500 && foo() && status_code < 500;
other < 1 && status_code > 500 && status_code < 400;

// Both sides make the same check
a == b && a == b;
a == b || a == b;
a == b && b == a;
a == (b) && (b) == a;
a != b || b != a;
!foo && !foo;
!foo || !foo;
foo && foo;
a.b && a.b;
(a && b) || (a && b);
a * b && b * a;
x < 5 && x < 5;

// One side is the opposite of the other
!foo && foo;
foo && !foo;
!foo || foo;
foo || !foo;
a == b && a != b;
a != b && b == a;
a !== a && a === a;
a === b || a !== b;
a !== a || a === a;
!(a === b) && a === b;

// Longer chains
x > 5 && x > 6 && x > 7;
x < 3 && x > 5 && x > 10;
(x = 1) && x > 5 && x < 3;
x > 5 && x < 10 && x > 3;
