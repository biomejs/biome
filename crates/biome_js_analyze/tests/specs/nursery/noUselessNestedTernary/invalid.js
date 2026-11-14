/* should generate diagnostics */

// The nested ternary shares its alternate with the outer ternary.
a ? (b ? c : d) : d;
a ? b ? c : d : d;
foo ? (bar ? baz : qux()) : qux();
foo ? (bar ? baz : object?.property) : object?.property;
foo ? (bar ? baz : "text") : 'text';
(foo || bar) ? (baz ?? qux ? value : fallback) : fallback;
const combined = foo || bar ? (baz ? value : fallback) : fallback;
foo ? (bar ? await baz : fallback) : fallback;
foo ? (bar ? baz = qux : fallback) : fallback;

// The nested ternary shares its consequent with the outer ternary.
a ? d : (b ? d : c);
a ? d : b ? d : c;
foo ? fallback : (bar || baz ? fallback : value);
foo ? fallback : (bar && baz ? fallback : value);
(foo ?? bar) ? fallback : (baz ? fallback : value);

// The shared result is in opposite branches.
a ? (b ? d : c) : d;
a ? c : (b ? d : c);
a ? (!b ? d : c) : d;
a ? (!(b || e) ? d : c) : d;
a ? (b === 1 ? d : c) : d;
outer ? (inner ?? other ? fallback : value) : fallback;
outer ? fallback : (inner || other ? value : fallback);
outer ? ((first, second) ? fallback : value) : fallback;
outer ? (counter++ ? fallback : value) : fallback;
async function run() { return outer ? (await check() ? fallback : value) : fallback; }
function* generate() { return outer ? fallback : ((yield check()) ? value : fallback); }

// A later branch has the same result as the first branch.
x === 1 ? a : (x === 2 ? b : (x === 3 ? a : c));
x === 1 ? a : x === 2 ? b : x === 3 ? a : c;
x === 1 ? a : (x === 2 ? b : (x === 3 ? d : (x === 4 ? a : c)));
x === 1 || x === 4 ? a : (x === 2 ? b : (x === 3 || x === 5 ? a : c));
x === 1 ? a : (x === 2 && enabled ? b : (x === 3 ? a : c));
x === 1 ? a : (enabled && (x === 2 || x === 4) ? b : (x === 3 ? a : c));
x === 1 ? a : (x === 2 ? b : (enabled && x === 3 ? a : c));
check() ? a : (x === 2 ? b : (x === 3 ? a : c));
first ?? second ? a : (x === 2 ? b : (x === 3 ? a : c));
1 === x ? a : (2 === x ? b : (3 === x ? a : c));
object.kind === "a" ? a : (object["kind"] === "b" ? b : (object[`kind`] === "c" ? a : c));
object[key] === 1 ? a : (object[key] === 2 ? b : (object[key] === 3 ? a : c));
object?.kind === 1 ? a : (object?.kind === 2 ? b : (object?.kind === 3 ? a : c));
this.kind === 1 ? a : (this.kind === 2 ? b : (this.kind === 3 ? a : c));
typeof x === "string" ? a : (typeof x === "number" ? b : (typeof x === "boolean" ? a : c));
x === -1 ? a : (x === 0 ? b : (x === +1 ? a : c));
x === 1n ? a : (x === 2n ? b : (x === 3n ? a : c));
x === null ? a : (x === false ? b : (x === true ? a : c));
x === 0 ? a : (x === null ? b : (x === "" ? a : c));
object.kind === "a" ? a : (object.kind === "b" ? b : (object.kind === "c" && object.data.ready ? a : c));
x === `a` ? a : (x === "b" ? b : (x === `c` ? a : c));
x === 1 ? a : (x === "1" ? b : (x === 1n ? a : c));
const multiline = x === 1
	? a
	: (x === 2
		? b
		: (x === 3 ? a : c));

// The first branch has the same result as the last alternate of the chain.
x === 1 ? c : (x === 2 ? b : c);
x === 1 ? a : (x === 2 ? c : (x === 3 ? b : c));
enabled && x === 1 ? c : (x === 2 ? a : (x === 3 ? b : c));

// The ternary nested in the first branch shares a result with the last alternate of the chain.
x === 1 ? (y ? a : c) : (x === 2 ? b : c);
x === 1 ? (y ? c : a) : (x === 2 ? b : c);
x === 1 ? (y ?? z ? c : a) : (x === 2 ? b : c);
enabled && x === 1 ? (y ? a : c) : (x === 2 ? b : c);

// Comments inside the parts that are kept don't prevent the fix.
check(/* keep */) ? (inner ? value : fallback) : fallback;
outer ? (inner(/* keep */) ? fallback : value) : fallback;
outer ? value : (inner ? value : fallback(/* keep */));
x === 1 ? a : (x === 2 /* keep */ ? b : (x === 3 ? a : c));
// leading comment
outer ? (inner ? value : fallback) : fallback; // trailing comment

// The fix would delete these comments, so it isn't offered.
foo ? (bar ? baz : fallback /* comment */) : fallback;
outer ? /* comment */ (inner ? value : fallback) : fallback;
outer ? value(/* comment */) : (inner ? value(/* comment */) : fallback);
x === 1 ? a : (x === 2 ? b : (x === 3 /* comment */ ? a : c));
x === 1 ? c /* comment */ : (x === 2 ? b : c);

// The fix would remove the parentheses that the `in` operator needs here, so it isn't offered.
for (let index = a ? (key in object ? 1 : 0) : 0; index < 3; index++) {}

// The fix would attach the expression to the previous line, so it isn't offered.
previous
first || second ? (third ? value : fallback) : fallback
