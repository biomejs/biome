// should generate diagnostics
declare const promise: Promise<number>;
declare const thenable: PromiseLike<number>;

promise["then"]((value) => value);
promise["then"]((value) => value)["finally"](() => {});
thenable["then"]((value) => value);
