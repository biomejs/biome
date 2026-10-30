/* should generate diagnostics */
Promise.resolve().then(() => log("a"));
await 1;
log("b");
