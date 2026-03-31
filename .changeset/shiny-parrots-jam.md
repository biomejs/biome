---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnsafeValues`](https://biomejs.dev/linter/rules/no-unsafe-values/), which disallows JSON values that are unsafe for interchange.

**Invalid:**

```json
[
  2e308, // Number evaluating to Infinity
  -2e308, // Number evaluating to -Infinity
  "\ud83d", // String with lone surrogate
  1e-400, // Unsafe zero (too small, will evaluate to 0)
  9007199254740992, // Unsafe integer (outside safe integer range)
  2.2250738585072009e-308, // Subnormal number
]
```
