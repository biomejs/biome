---
"@biomejs/biome": patch
---

Added the nursery rule [`noCollectionBracketAccess`](https://biomejs.dev/linter/rules/no-collection-bracket-access/). It reports square brackets used on a `Map`, `Set`, `WeakMap`, or `WeakSet`, which read or change a property of the object instead of one of its entries.

```js
const map = new Map();
map["foo"] = "bar";
```
