// should not generate diagnostics
declare const promise: Promise<number>;
declare const thenable: PromiseLike<number>;

promise["then"]((value) => value, () => {});
promise["catch"](() => {});
promise.then((value) => value)["catch"](() => {});
promise["then"]((value) => value, () => {})["finally"](() => {});
thenable["then"]((value) => value, () => {});
thenable.then((value) => value, () => {});
