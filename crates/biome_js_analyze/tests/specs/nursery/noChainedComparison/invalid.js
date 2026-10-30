/* should generate diagnostics */
if (a < b < c) {}
if (a > b > c) {}
if (a <= b >= c) {}
if (a >= b < c) {}
if (a == b == c) {}
if (a === b === c) {}
if (a != b !== c) {}
if (a !== b == c) {}

// Parentheses don't change the result, and the formatter adds them to equality chains.
if ((a == b) == c) {}
if ((a < b) < c) {}

// One diagnostic for the whole chain.
if (a == b == c == d) {}
if ((a == b == c) == d) {}
if (a < b < c < d) {}

// The right side is not clearly a boolean.
if (a === b === foo()) {}
if (a === b === c.d) {}
const x = a < b < 10;

// Comparing booleans with `<` or `>` is still reported.
if ((a < b) < (c < d)) {}
if (a < b < !c) {}
if (a < b < true) {}
