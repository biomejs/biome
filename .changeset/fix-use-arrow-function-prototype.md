---
"@biomejs/biome": patch
---

Fixed [#12372](https://github.com/biomejs/biome/issues/12372): [`useArrowFunction`](https://biomejs.dev/linter/rules/use-arrow-function/) no longer reports a function expression assigned to a variable when the variable is used in a way that needs a `prototype`: `f.prototype`, `new f()`, `extends f`, or `instanceof f`. Arrow functions don't have a `prototype`, so the safe fix changed the behavior of code such as the following:

```js
const f = function () {};
console.log(f.prototype.toString());
```
