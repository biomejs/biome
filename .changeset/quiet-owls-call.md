---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessCall`](https://biomejs.dev/linter/rules/no-useless-call/), which reports `.call()` and `.apply()` calls that can be replaced with a regular function call.

```js
obj.foo.call(obj, 1, 2);
foo.apply(null, [1, 2]);
```
