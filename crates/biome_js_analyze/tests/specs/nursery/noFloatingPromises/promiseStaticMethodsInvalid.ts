// should generate diagnostics
const promises = [Promise.resolve(1), Promise.resolve(2)];

Promise.allSettled(promises);
Promise.any(promises);
Promise.race(promises);
Promise.try(() => 1);
