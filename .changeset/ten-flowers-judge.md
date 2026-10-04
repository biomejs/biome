---
"@biomejs/biome": patch
---

Added the nursery rule [`noInvalidThis`](https://biomejs.dev/linter/rules/no-invalid-this/), which reports `this` where it is `undefined`: at the top level of a module, and in functions that aren't methods or constructors.

```js
function foo() {
  this.a = 0;
}
```
