// While loops
async function foo() { while (baz) { await bar; } }
async function foo() { while (await foo()) {  } }
async function foo() { while (baz) { for await (x of xs); } }
// For of loops
async function foo() { for (var bar of baz) { await bar; } }
async function foo() { for (var bar of baz) await bar; }
// For in loops
async function foo() { for (var bar in baz) { await bar; } }
// For loops
async function foo() { for (var i; i < n; i++) { await bar; } }
async function foo() { for (var i; await foo(i); i++) {  } }
async function foo() { for (var i; i < n; i = await bar) {  } }
// Do while loops
async function foo() { do { await bar; } while (baz); }
async function foo() { do { } while (await bar); }
// Deep in a loop body
async function foo() { while (true) { if (bar) { foo(await bar); } } }
// Deep in a loop condition
async function foo() { while (xyz || 5 > await x) {  } }
// In a nested loop of for-await-of
async function foo() { for await (var x of xs) { while (1) await f(x) } }
