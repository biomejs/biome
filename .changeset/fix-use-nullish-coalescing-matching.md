---
"@biomejs/biome": patch
---

[`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) now matches ternary and `if` subjects through optional chaining and bracket access, skips subjects that contain calls, and withholds fixes that would drop comments, cross a type assertion, or join the previous statement.
