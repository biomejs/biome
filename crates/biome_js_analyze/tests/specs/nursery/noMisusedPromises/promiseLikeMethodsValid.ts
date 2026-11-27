/* should not generate diagnostics */
declare const thenable: PromiseLike<number>;
declare const resource: AsyncDisposable;
declare const fallback: boolean;

if (await thenable.then((value) => value)) {
}

const chained = await thenable.then((value) => value);
if (chained || fallback) {
}

const disposed = await resource[Symbol.asyncDispose]();
if (disposed || fallback) {
}
