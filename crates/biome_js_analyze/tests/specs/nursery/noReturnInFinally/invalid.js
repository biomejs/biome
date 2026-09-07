/* should generate diagnostics */

Promise.resolve(1).finally(() => { return 2 })

Promise.reject(0).finally(() => { return 2 })

myPromise.finally(() => { return 2 });

Promise.resolve(1).finally(function () { return 2 })