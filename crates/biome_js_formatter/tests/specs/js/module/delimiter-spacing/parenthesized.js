// =====================
// Single line (hugging)
// =====================

// Type cast comments preserve parentheses - objects get spaces
const foo1 = /** @type {Foo} */ ({});
const foo2 = /** @type {Foo} */ ({ a: 1 });
const foo3 = /** @type {Foo} */ ({ a: 1, b: 2 });

// Type cast comments preserve parentheses - arrays get spaces
const foo4 = /** @type {Foo} */ ([]);
const foo5 = /** @type {Foo} */ ([1, 2, 3]);
const foo6 = /** @type {Foo} */ ([a, b, c]);

// =====================
// Contrast cases (no hugging)
// =====================

// With comments inside, hugging is disabled (no spaces)
const foo7 = /** @type {Foo} */ (/* comment */ {});
const foo8 = /** @type {Foo} */ (/* comment */ []);

// Non-object/array expressions don't hug (no spaces)
const foo9 = /** @type {Foo} */ (b);
const foo10 = /** @type {Foo} */ (b + c);

// =====================
// Boundary tests
// =====================

// Boundary: 78 chars without spaces, 80 with spaces (fits on one line)
const foo11 = /** @type {Foo} */ ({ ooooooooooooooooooooooooooooooooooo: 1 });

// Boundary: 79 chars without spaces, 81 with spaces (breaks to multiple lines)
const foo12 = /** @type {Foo} */ ({ oooooooooooooooooooooooooooooooooooo: 1 });
