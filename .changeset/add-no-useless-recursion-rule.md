---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessRecursion`](https://biomejs.dev/linter/rules/no-useless-recursion/), which reports functions that return a call to themselves when a loop can do the same work. Too many nested calls can crash the program with a stack overflow error.

```js
function findRoot(node) {
    if (node.parent) {
        return findRoot(node.parent);
    }
    return node;
}
```
