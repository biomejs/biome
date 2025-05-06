/* should not generate diagnostics */
var foo = bar();
foo.void();
foo.void = bar;
delete foo;
