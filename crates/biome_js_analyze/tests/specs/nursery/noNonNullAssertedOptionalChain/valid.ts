/* should not generate diagnostics */

// Non-null assertion without optional chaining
foo.bar!;

// Non-null assertion followed by optional chaining
foo.bar!.baz;

// Non-null assertion on function call
foo.bar()!;

// Non-null assertion on function call followed by method call
foo.bar()!();

// Non-null assertion on function call followed by property access
foo.bar()!.baz;

// Optional chaining without non-null assertion
foo?.bar;

// Optional chaining method call without non-null assertion
foo?.bar();

// Parenthesized optional chaining without non-null assertion on the chain
(foo?.bar).baz!;

// Parenthesized optional chaining call without non-null assertion on the chain
(foo?.bar()).baz!;

// Logical AND with chaining patterns
foo && foo.bar;

// Logical OR pattern
foo || foo.bar;