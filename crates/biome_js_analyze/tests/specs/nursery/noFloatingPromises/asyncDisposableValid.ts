// should not generate diagnostics
declare const resource: AsyncDisposable;

await resource[Symbol.asyncDispose]();
void resource[Symbol.asyncDispose]();
const disposed = await resource[Symbol.asyncDispose]();
disposed;
[disposed];
