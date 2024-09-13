// Not length check
if (foo.notLength) {}
if (length) {}
if (foo[length]) {}
if (foo["length"]) {}

// Not boolean
const bar = foo.length
const bar1 = +foo.length
const x0 = Boolean(foo.length, foo.length)
const x1 = new Boolean(foo.length)
const x2 = NotBoolean(foo.length)
const length = foo.length ?? 0
if (foo.length ?? bar) {}


// Valid style
foo.length === 0
foo.length > 0
while (foo.length > 0) {
    foo.pop();
}
if (foo.length > 0) {}
for (; foo.length > 0; foo.pop()); {}

if (foo.length !== 1) {}
if (foo.length > 1) {}
if (foo.length > 1) {}
if (foo.length < 2) {}

if (foo.length === 0 || true) {}

// TODO. Not supported yet.
// With known static length value
// const foo1 = { size: "small" }; if (foo1.size) {} // Not a number
// const foo2 = { length: -1 }; if (foo2.length) {} // Array lengths cannot be negative
// const foo3 = { length: 1.5 }; if (foo3.length) {} // Array lengths must be integers
// const foo4 = { length: NaN }; if (foo4.length) {} // Array lengths cannot be NaN
// const foo5 = { length: Infinity }; if (foo5.length) {} // Array lengths cannot be Infinity

// Logical OR
const x3 = foo.length || 2
// TODO. Not supported yet.
// const A_NUMBER = 2; const x = foo.length || A_NUMBER
