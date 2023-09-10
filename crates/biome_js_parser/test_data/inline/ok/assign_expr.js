foo += bar = b ??= 3;
foo -= bar;
(foo = bar);
[foo, bar] = baz;
[foo, bar = "default", ...rest] = baz;
[,,,foo,bar] = baz;
({ bar, baz } = {});
({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
