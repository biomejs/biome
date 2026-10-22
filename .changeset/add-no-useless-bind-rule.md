---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessBind`](https://biomejs.dev/linter/rules/no-useless-bind/), which disallows `.bind()` calls on functions that don't use `this`, and on arrow functions.

```js
const handler = function () {
  console.log("clicked");
}.bind(this); // reported
```
