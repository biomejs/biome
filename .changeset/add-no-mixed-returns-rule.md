---
"@biomejs/biome": patch
---

Added the nursery rule [`noMixedReturns`](https://biomejs.dev/linter/rules/no-mixed-returns/), which reports functions where some `return` statements return a value and others don't, including functions that can reach the end of their body after returning a value elsewhere.

```js
function doSomething(condition) {
    if (condition) {
        return true;
    }
}
```
