/* should generate diagnostics */
// Not fixable: without `await`, the code would start with a block.
async function f1() { await {a}.b++; }
const f2 = async () => await {a}.b++;
