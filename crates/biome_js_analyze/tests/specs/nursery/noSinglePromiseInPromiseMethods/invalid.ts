/* should generate diagnostics */
await Promise.race([value as Promise<number>]);
await Promise.any([promise!]);
async function run(promise: Promise<number>) { return await Promise.any<number>([promise!]); }
Promise.race<number>([promise]);
await Promise.race([promise satisfies Promise<number>]);

// Reported without a fix.
const [foo]: [Foo] = await Promise.all([promise]);
