---
"@biomejs/biome": patch
---

Added the nursery rule [`useEarlyContinue`](https://biomejs.dev/linter/rules/use-early-continue/), which reports an `if` statement that wraps the rest of a loop body and suggests skipping the iteration early with `continue` instead.

```js
for (const item of items) {
  if (item.isActive) {
    process(item);
    save(item);
  }
}
```
