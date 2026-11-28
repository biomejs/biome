---
"@biomejs/biome": patch
---

Added the nursery rule [`noBreakInNestedLoop`](https://biomejs.dev/linter/rules/no-break-in-nested-loop/), which reports `break` statements without a label inside nested loops and inside `switch` statements in loops. Enable the `checkContinue` option to also report `continue` statements without a label.

```js
for (const item of items) {
  for (const child of item.children) {
    break; // reported
  }
}
```
