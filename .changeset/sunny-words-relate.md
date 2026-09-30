---
"@biomejs/biome": patch
---

Fixed [#11898](https://github.com/biomejs/biome/issues/11898): [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return/) no longer offers a safe fix for a `return;` that is the body of an unbraced `if`, `else`, or label, because removing it produced invalid code. The safe fix now also keeps comments placed before or after the removed `return;`.

```js
function foo() {
  // Still reported, but the return is no longer removed
  if (aborted) return;
}
```
