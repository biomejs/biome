---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return/). The rule reports redundant `return;` statements that don't affect the function's control flow.

```js
// Invalid: return at end of function is redundant
function foo() {
    doSomething();
    return;
}
```
