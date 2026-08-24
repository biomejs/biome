---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnmodifiedLoopCondition`](https://biomejs.dev/linter/rules/no-unmodified-loop-condition/), which reports variables in loop conditions that are never modified in the loop.

```js
let node = getNode();
while (node) {
    process(node);
}
```
