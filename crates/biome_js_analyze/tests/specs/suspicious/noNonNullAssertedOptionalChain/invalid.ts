// https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/tests/rules/no-non-null-asserted-optional-chain.test.ts

// Basic optional chaining with non-null assertion
foo?.bar!;

// Computed member access with optional chaining and non-null assertion
foo?.["bar"]!;

// Optional chaining method call with non-null assertion
foo?.bar()!;

// Optional chaining call with non-null assertion
foo.bar?.()!;

// Parenthesized optional chaining with non-null assertion
(foo?.bar)!.baz;

// Parenthesized optional chaining call with non-null assertion
(foo?.bar)!().baz;

// Parenthesized optional chaining with non-null assertion (standalone)
(foo?.bar)!;

// Parenthesized optional chaining call with non-null assertion (standalone)
(foo?.bar)!();

// Nested parenthesized optional chaining with non-null assertion
(foo?.bar!);

// Nested parenthesized optional chaining call with non-null assertion
(foo?.bar!)();
