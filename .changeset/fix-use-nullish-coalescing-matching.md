---
"@biomejs/biome": patch
---

[`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) now matches ternary and `if` subjects through optional chaining and bracket access, and only when the subject is a variable or property; reports `||` that reaches a condition only through a call argument, a function body, or a unary operator other than `!`; ignores `||=` in a condition when `ignoreConditionalTests` is enabled; reports `if` bodies written as `a ||= b`, `a ??= b`, or `(a = b);`; limits `ignoreBooleanCoercion` to ternary branches, sequences, and the first argument of `Boolean()`; applies `ignorePrimitives` to the type of the tested value; and withholds fixes that would drop comments, cross a type assertion, or join the previous statement.
