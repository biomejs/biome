---
"@biomejs/biome": patch
---

Added a new nursery rule [`useStringStartsEndsWith`](https://biomejs.dev/linter/rules/use-string-starts-ends-with/), which prefers `startsWith()` and `endsWith()` over verbose string prefix and suffix checks.

The rule uses type information, so it only reports on strings and skips array lookups such as `items[0] === "a"`.
