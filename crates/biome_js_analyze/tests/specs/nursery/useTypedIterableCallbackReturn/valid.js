/* should not generate diagnostics */
function log(value) { console.log(value); }
[1, 2, 3].forEach(value => log(value));
[1, 2, 3].forEach(function(value) { return log(value); });
[1, 2, 3].map(() => 42);
[1, 2, 3].forEach(() => void 0);
[1, 2, 3].forEach(() => undefined);
[1, 2, 3].map(() => undefined);
[1, 2, 3].map(() => { throw new Error(); });
[1, 2, 3].map(async () => {});
[1, 2, 3].map(function* () {});
[1, 2, 3].forEach(async () => 42);
[1, 2, 3].forEach(function* () { yield 42; });
[1, 2, 3].map(() => { function nested() { return; } return 42; });
[1, 2, 3].forEach(() => { function nested() { return 42; } });
const custom = { every(callback) { callback(); } };
custom.every(() => {});
