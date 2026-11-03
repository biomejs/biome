---
"@biomejs/biome": patch
---

Added the new nursery rule [`useObjectFromEntries`](https://biomejs.dev/linter/rules/use-object-from-entries/), which reports code that builds an object from a list of key-value pairs with `reduce()`, a `for...of` loop, or `_.fromPairs()` instead of `Object.fromEntries()`. The `functions` option adds other helper functions to report.

```js
const object = pairs.reduce((object, [key, value]) => ({ ...object, [key]: value }), {});
```
