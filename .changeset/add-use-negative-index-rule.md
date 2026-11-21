---
"@biomejs/biome": patch
---

Added the nursery rule [`useNegativeIndex`](https://biomejs.dev/linter/rules/use-negative-index/), which reports index arguments of `slice()`, `at()`, `subarray()`, `splice()`, and `toSpliced()` that subtract a number from `.length`, and suggests a negative index instead.

```js
foo.at(foo.length - 1); // reported: write `foo.at(-1)` instead
```
