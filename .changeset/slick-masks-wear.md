---
"@biomejs/biome": patch
---

Added new nursery rule [`noReturnAssign`](https://biomejs.dev/linter/rules/no-return-assign), which disallows assignments inside return statements.

**Invalid:**

```js
function f(a) {
    return a = 1;
}
```
