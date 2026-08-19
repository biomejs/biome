// should generate diagnostics
const promises = [Promise.resolve(1), Promise.resolve(2)];

if (Promise.allSettled(promises)) {}
if (Promise.any(promises)) {}
if (Promise.race(promises)) {}
if (Promise.try(() => 1)) {}
