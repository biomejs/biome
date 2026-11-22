---
"@biomejs/biome": patch
---

Added the nursery rule [`useElseIf`](https://biomejs.dev/linter/rules/use-else-if/). It reports an `if` statement that follows another `if` statement and compares the same variable or property against a different value, since only one of them can match. The rule offers an unsafe fix that adds `else` before the second `if` statement.

```js
if (status === "loading") {}
if (status === "error") {}
```
