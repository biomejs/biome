const condition = Math.random() > -1; // Always true, but dynamic to linter
condition ? Promise.reject("ternary bypass") : null;

// A test that is always falsy keeps the promise in the type of the expression,
// as in TypeScript.
const alwaysFalsy = 0;
alwaysFalsy ? Promise.reject("ternary bypass") : null;
