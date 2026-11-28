/* should generate diagnostics */
declare const thenable: PromiseLike<number>;
declare const resource: AsyncDisposable;

if (thenable.then((value) => value)) {
}

if (resource[Symbol.asyncDispose]()) {
}
