/* should generate diagnostics */
// Fixable: nothing runs after the `await`.
async function f1() { await 1; }
async function f2() { return await 1; }
async function f3() { const x = await 1; }
async function f4() { if (q) { await 1; } }
async function f5() { if (q) { run(); } else { await 1; } }
async function f6() { log("s"); await 1; }
const f7 = async () => { await 1; };
const f8 = async () => await 1;
async function f9() { return (await 1); }
async function f10() { return (0, await 1); }
async function f11() { return await/* comment */1; }
async function f12() {
	return await
		// comment
		1;
}
async function f13() {
	throw await
		// comment
		[];
}
async function f14() {
	return( await
		// comment
		1);
}
async function f17() {
	return await // comment
		[1, 2];
}
async function f15() { await (function () {}); }
async function f16() { return await function () {}; }
async function outer() { const f = async () => await 1; run(); }

// Not fixable: something runs after the `await`.
async function g1() { log("s"); await 1; log("e"); }
async function g2() { if (q) { await 1; log("e"); } }
async function g3() { log(await 1); }
async function g4() { for (const x of xs) { await 1; } }
async function g5() { try { await 1; } finally { log("e"); } }
async function g6() { if (await 1) {} }
async function g7() { const x = await 1, y = 2; }
async function g8() { await using x = await 1; }
async function g9() { await 1 + 1; }
async function g10() { label: { await 1; } }
async function g11() { using r = get(); await 1; }
async function g12() { const [a = log("e")] = await []; }
async function g13() { const { a } = await 1; }
async function outer2() { return async () => (await 1, log("e")); }
