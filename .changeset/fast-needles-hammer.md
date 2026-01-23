---
"@biomejs/biome": patch
---

Added the nursery rule [`noDivRegex`](https://biomejs.dev/linter/rules/no-div-regex). Disallow equal signs explicitly at the beginning of regular expressions.

**Invalid:**

```js
var f = function() {
  return /=foo/;
};
```
