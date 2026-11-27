---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateIfBranches`](https://biomejs.dev/linter/rules/no-duplicate-if-branches/), which reports branches of an `if` statement that run the same code as the branch right before them.

```js
if (value === "a") {
  doThing();
} else if (value === "b") {
  doThing();
}
```
