/* should generate diagnostics */
async function f1() { return await (1 as number); }
async function f2() { return await ([1, 2]!); }
async function f3() { return await ([1, 2] as const); }
async function f4() { return await ([1, 2] satisfies number[]); }
// No fix: without `await`, these would start a declaration.
async function f5() { await function () {}!; }
async function f6() { await class {}!; }
