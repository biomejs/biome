// should generate diagnostics
const promise = Promise.resolve(true);

[1, 2, 3].find(() => promise);
[1, 2, 3].every(() => promise);
