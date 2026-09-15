---
"@biomejs/biome": patch
---

Added the nursery rule [`useStrictBooleanExpressions`](https://biomejs.dev/linter/rules/use-strict-boolean-expressions/), which reports ambiguous truthiness checks such as `if (value)` when `value` has type `number | undefined`. Non-nullable strings and numbers and nullable objects are allowed; the rule has no options.
