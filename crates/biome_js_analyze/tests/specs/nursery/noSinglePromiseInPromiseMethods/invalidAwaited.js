/* should generate diagnostics */
await Promise.race([promise]);
await Promise.race([promise,],);
await Promise.any([promise]);
await Promise.all([promise]);
const a = await Promise.race([promise]);
const b = await Promise.any([promise]);
async function c() { return await Promise.race([getPromise()]); }
await Promise.race([(0, promise)]);
await Promise.race([a ? b : c]);
await Promise.race([() => promise]);
await Promise.race([x ??= y]);
await Promise.race([x + y]);
await Promise.race([x ** y]);
await Promise.race([promises[0]]);
await Promise.race([await promise]);
await Promise.race([new Promise(() => {})]);
+await Promise.race([+1]);
async function* d() { await Promise.race([yield promise]); }
await(Promise).race([read()]);
await (Promise.race([promise]));
async function e() { return delete await Promise.race([object.value]); }
await globalThis.Promise.race([promise]);
await Promise["race"]([promise]);
await window.Promise.any([promise]);
