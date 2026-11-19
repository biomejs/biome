/* should generate diagnostics */

// Object destructuring shadowing outer variable
const x = 1;
function shadowObj() {
    const { a: x } = { a: 2 };
}

// Array destructuring shadowing outer variable
const y = 1;
function shadowArr() {
    const [y] = [2];
}

// Nested destructuring shadowing outer variable
const z = 1;
function shadowNested() {
    const { a: { b: z } } = { a: { b: 2 } };
}

// Shorthand object destructuring shadowing outer variable
const w = 1;
function shadowShorthand() {
    const { w } = { w: 2 };
}

// Mixed nested destructuring shadowing outer variable
const m = 1;
function shadowMixed() {
    const [{ m }] = [{ m: 2 }];
}

// Rest in array destructuring shadowing outer variable
const rest = 1;
function shadowRestArr() {
    const [, ...rest] = [1, 2, 3];
}

// Rest in object destructuring shadowing outer variable
const other = 1;
function shadowRestObj() {
    const { a, ...other } = { a: 1, b: 2 };
}
