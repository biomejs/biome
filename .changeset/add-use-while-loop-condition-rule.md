---
"@biomejs/biome": patch
---

Added the new nursery rule [`useWhileLoopCondition`](https://biomejs.dev/linter/rules/use-while-loop-condition/), which reports loops such as `while (true)` and `for (;;)` that start with an `if` statement containing only a `break`, and moves that condition into the `while` statement.

```js
while (true) {
  if (!hasMore()) {
    break;
  } // reported
  processNext();
}
```
