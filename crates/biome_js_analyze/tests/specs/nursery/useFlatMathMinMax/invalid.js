/* should generate diagnostics */

Math.max(Math.max(a, b), c);
Math.min(a, Math.min(b, c));
Math.max(Math.max(a, b), Math.max(c, d), e);
Math.min(Math.min(Math.min(a, b), c), d);
Math.max(Math.max(a, b));
Math.min(Math.min());
Math.max(Math.max(...values), fallback);
const value = Math.max(foo, Math.max(bar, baz)).toString();
const parenthesized = Math.min((Math.min(a, b)), c);
Math.max(foo(), Math.max(bar(), baz()), qux());
Math.min(Math.min(foo(a), bar(b)), baz(Math.min(c, d)));

const multiline = Math.max(
    Math.max(
        a,
        b,
    ),
    c,
);

Math.max(/* keep */ Math.max(a, b), c);
Math.max(Math.max(a, b) /* keep */, c);
Math.max(Math.max(/* keep */ a, b), c);
Math.max(Math.max(a, b), /* keep */ c);
Math./* keep */max(Math.max(a, b), c);
