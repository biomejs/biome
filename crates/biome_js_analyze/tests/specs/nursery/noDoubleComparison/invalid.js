/* should generate diagnostics */
x === y || x < y;
x < y || x === y;
x === y || x > y;
x > y || x === y;
x <= y && x >= y;
x >= y && x <= y;
x <= y && x !== y;
x !== y && x <= y;
x >= y && x !== y;
x !== y && x >= y;
x < y || x > y;
x > y || x < y;
undefined === undefined || undefined < undefined;
undefined <= undefined && undefined >= undefined;
const symbol = Symbol();
symbol === symbol || symbol < symbol;
symbol <= symbol && symbol >= symbol;
null <= 0 && null !== 0;
null >= 0 && null !== 0;
x === y || y < x;
y < x || x === y;
x <= y && y <= x;
a.b === c || c > a.b;
x === 5 || x < 5;
x === "a" || x < 'a';
a.b === c || a.b < c;
this.x === y || this.x > y;
a[b] === c || a[b] > c;
a[0] === c || a[0] > c;
this.#x === y || this.#x < y;
a.b === c || a["b"] < c;
a["b"] === c || a[`b`] > c;
a[0] === c || a["0"] < c;
a[1.5] === c || a["1.5"] < c;
a.b.c === d || a["b"][`c`] < d;
a[i].b === c || a[i]["b"] < c;
x === y || x < y || z;
x < y || x === y || x > y;
(x === y || x < y).foo;
if (x === y || x < y) {}
const result = x >= y && x !== y;
(x === y) || (x < y);
(a /* kept */) === c || a < c;
let regex = /foo/;
regex === b || regex < b;
// Variables that refer to each other are followed only once.
const loopA = loopB, loopB = loopA;
loopA === c || loopA < c;
// Escaped and non-decimal property names.
a["\x62"] === c || a.b < c;
a[0x1000000000000081] === c || a["1152921504606847200"] < c;
// Escapes that are compared by their source text.
x === "\01" || x < "\01";
a["\1"] === c || a["\1"] < c;
