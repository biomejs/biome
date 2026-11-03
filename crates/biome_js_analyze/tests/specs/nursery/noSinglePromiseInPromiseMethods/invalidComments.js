/* should generate diagnostics */
// Fixed, the comments are kept.
await Promise.race([read(/* keep */)]);
await Promise.race([(/* keep */ promise)]);
const [a] = await Promise.all([(/* keep */ promise)]);
const b = (await Promise.all([read(/* keep */)]))[0];
/* keep */ await Promise.race([promise]) /* keep */;
Promise.race([read(/* keep */)]);
Promise./* keep */race([promise]);

// Reported without a fix, a comment would be lost.
await Promise./* lost */race([promise]);
await Promise.race([/* lost */ promise]);
await Promise.race([promise /* lost */]);
await Promise.race([promise, /* lost */]);
await Promise.race([
	// lost
	promise,
]);
await /* lost */ Promise.race([promise]);
const [/* lost */ c] = await Promise.all([promise]);
const d = (await Promise.all([promise]) /* lost */)[0];
const e = (await Promise.all([promise]))[/* lost */ 0];
Promise.race([/* lost */ promise]);
