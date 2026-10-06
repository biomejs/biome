/* should not generate diagnostics */
// Loose equality is not combined.
x == y || x < y;
x == y || x > y;
x != y && x <= y;
// One comparison already covers the other.
x <= y || x < y;
x < y || x <= y;
x >= y || x > y;
x === y || x <= y;
// Different values.
x === y || x < z;
a === b || c < d;
// Same operator on both sides.
x < y || x < y;
x === y || x === y;
// Pairs that don't combine into a single comparison.
x <= y || x >= y;
x < y || y > x;
x > y || y < x;
x <= y && y >= x;
x < y && x > y;
x <= y || x > y;
x === y || x !== y;
x === y && x !== y;
// Function calls can return a different value each time.
f() === y || f() < y;
x === f() || x < f();
// The comparisons are not next to each other.
x === y || z || x < y;
// Optional chaining.
x?.a === y || x?.a < y;
x?.a <= y && x?.a >= y;
a?.b.c === y || a?.b.c < y;
a[b?.c] === y || a[b?.c] < y;
// Not both comparisons.
x === y || foo(x, y);
x === y || x;
x === y ?? x < y;
// Different properties.
a.b === c || a.d < c;
a[0] === c || a[1] < c;
// Regular expressions are objects.
/foo/ === y || /foo/ < y;
x === /foo/ || x < /foo/;
a.b === c || a["c"] < c;
a[1e21] === c || a["1e21"] < c;
a[i] === c || a["i"] < c;
class Private {
	#b;
	method() {
		this.#b === c || this["#b"] < c;
	}
}
x === "\1" || x < "1";
x === "\01" || x < "\001";
a["\01"] === c || a.b < c;
a["\uD800"] === c || a["\uDC00"] < c;
a["\u{D800}"] === c || a["\u{DC00}"] < c;
a[0x1000000000000081] === c || a["1152921504606847000"] < c;
