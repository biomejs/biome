/* should generate diagnostics */
obj.readSync!();
(fs.readFileSync as Reader)(path);
fs.fooSync!.call(null);
(fooSync satisfies Reader)();
