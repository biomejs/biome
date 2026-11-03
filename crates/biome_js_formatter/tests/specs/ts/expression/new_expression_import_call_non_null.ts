// `new import("foo")!()` is a syntax error, so the parentheses must stay.
new (import("foo")!)();
new (import("foo")!.bar)();
