/* should not generate diagnostics */
async function returnsPromise(): Promise<string> {
  return 'value';
}

await returnsPromise();
void returnsPromise();
return returnsPromise();

returnsPromise().then(
  () => { },
  () => { },
);

returnsPromise().catch(() => { });


const promise = new Promise((resolve) => resolve('value'));
promise.then(() => { }, () => { })

Promise.resolve('value').then(() => { }, () => { })
Promise.all([p1, p2, p3]).catch(() => { })

const Promise = { resolve(): {} };
Promise.resolve()
async function bindingPromiseInsideFunction() {
  Promise.resolve()
}