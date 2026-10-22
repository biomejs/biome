---
"@biomejs/biome": patch
---

Added the nursery rule [`useMathTrunc`](https://biomejs.dev/linter/rules/use-math-trunc/), which requires `Math.trunc()` instead of bitwise operations or `parseInt(String(x), 10)` for removing the decimal part of a number. For example, `const result = value | 0;` and `const result = ~~value;` are now reported.
