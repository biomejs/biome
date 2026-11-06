---
"@biomejs/biome": patch
---

Added the new nursery rule [`noSubtractionComparison`](https://biomejs.dev/linter/rules/no-subtraction-comparison/), which reports subtracting two values only to compare the difference with `0`, and suggests comparing the values directly.

```js
if (a - b > 0) {}
```
