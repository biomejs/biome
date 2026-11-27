---
"@biomejs/biome": patch
---

Added the nursery rule [`noAmd`](https://biomejs.dev/linter/rules/no-amd/), which reports AMD `define()` and `require()` calls that load modules from a list, so they can be replaced with `import` statements.

```js
define(["a", "b"], function (a, b) {});
```
