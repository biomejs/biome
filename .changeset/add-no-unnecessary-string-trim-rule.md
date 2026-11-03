---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnnecessaryStringTrim`](https://biomejs.dev/linter/rules/no-unnecessary-string-trim/), which reports `trim()` calls before `startsWith()` or `endsWith()` when only one side of the string matters. Use `trimStart()` or `trimEnd()` instead.

```js
value.trim().startsWith("-");
```
