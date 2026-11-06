// `new import("foo").bar()` is a syntax error, so the parentheses must stay.
new (import("foo"))();
new (import("foo").bar)();
new (import("foo")["bar"])();
new (import("foo")`bar`)();
