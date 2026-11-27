/* should generate diagnostics */
Promise.all([promise]);
const a = () => Promise.all([promise]);
const b = await Promise.all([promise]);
c = await Promise.all([promise]);
const obj = { p: Promise.all([promise]) };
const [d] = await Promise.all([promise]);
let [e,] = await Promise.all([promise]);
var[f] = await Promise.all([promise]);
[g] = await Promise.all([promise]);
const h = (await Promise.all([promise]))[0];
const i = (await Promise.all([promise]))[0.0];
j = (await Promise.all([promise]))[0];
const [k] = await Promise.all([a ? b : c]);
if (condition) {} else[l] = await Promise.all([promise]);

// Reported without a fix.
const [m = fallback] = await Promise.all([promise]);
const [...n] = await Promise.all([promise]);
const [, o] = await Promise.all([promise]);
const p = ([q] = await Promise.all([promise]));
const r = (await Promise.all([promise]))[1];
const s = (await Promise.all([promise]))?.[0];
foo((await Promise.all([promise]))[0]);
return2 += await Promise.all([promise]);
