/* should not generate diagnostics */
async function f1() { return await (a as Promise<number>); }
async function f2() { return await (a!); }
async function f3() { return await (<Promise<number>>a); }
