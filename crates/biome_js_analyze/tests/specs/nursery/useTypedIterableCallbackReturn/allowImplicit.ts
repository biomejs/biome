/* should not generate diagnostics */
function log(): void {}
[1].map(() => { return; });
[1].map(() => log());
[1].map(() => { return log(); });
[1].map(() => { if (condition) return 42; return; });
