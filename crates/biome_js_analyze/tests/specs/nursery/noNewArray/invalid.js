/* should generate diagnostics */
// A number or a value that may be a number: reported without a fix.
const a1 = new Array(1);
const a2 = new Array(length);
const a3 = new Array(foo.length);
const a4 = new Array(-1);
const a5 = new Array(1.5);
const a6 = new Array(0xff);
const a7 = new Array(foo || 1);
const a8 = new Array(foo ? "x" : 1);
const a9 = new Array(-foo);
const a10 = new Array(+foo);
const a11 = new Array((0, 1));
const a12 = new Array(Number("1"));
const a13 = new Array(undefined);
const a14 = new Array(String.raw`x`);
const a15 = new Array(foo + "x");
const a16 = new Array(new Array(1)[0]);

// Never a number: fixed to an array literal.
const b1 = new Array("1");
const b2 = new Array(("1"));
const b3 = new Array(null);
const b4 = new Array(true);
const b5 = new Array(1n);
const b6 = new Array(`x`);
const b7 = new Array(`x${foo}`);
const b8 = new Array({ value: 1 });
const b9 = new Array([foo]);
const b10 = new Array(() => {});
const b11 = new Array(function () {});
const b12 = new Array(class {});
const b13 = new Array(/x/);
const b14 = new Array(new Number(1));
const b15 = new Array(typeof foo);
const b16 = new Array(!foo);
const b17 = new Array(void foo);
const b18 = new Array(foo instanceof Bar);
const b19 = new Array("x" in foo);
const b20 = new Array(foo === 1);
const b21 = new Array((0, "x"));
const b22 = new Array("x",);
const b23 = new Array(import("x"));

// Other ways to reference the global `Array`.
const c1 = new (Array)("x");
const c2 = new globalThis.Array("x");
const c3 = new window.Array("x");

// Comments.
const d1 = /* kept */ new Array("x") /* kept */;
const d2 = new Array(/* kept */ "x" /* kept */);
const d3 = new Array(
	// kept
	"x"
	// kept
);
const d4 = new /* removed */ Array("x");
const d5 = new Array /* removed */ ("x");
const d6 = new Array(/* kept */ 1);
const d7 = new Array("x", /* kept */);
const d8 = new Array(
	"x", // kept
);

// Placement.
function f() {
	return new Array("x");
}
new Array("x").forEach(g);
foo();
new Array("x").forEach(g);
foo()
new Array("x").forEach(g);
