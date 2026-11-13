---
"@biomejs/biome": patch
---

Fixed [#13129](https://github.com/biomejs/biome/issues/13129): the fix of [`useArrowFunction`](https://biomejs.dev/linter/rules/use-arrow-function/) no longer produces invalid code when the function is in the initializer of a `for` statement and returns an `in` expression. The arrow function body is now wrapped in parentheses.

```js
// Before
for (let has = function (k) { return k in cache; }; ;) {}
// After the fix
for (let has = (k) => (k in cache); ;) {}
```
