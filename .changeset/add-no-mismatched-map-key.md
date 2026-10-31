---
"@biomejs/biome": patch
---

Added the nursery rule [`noMismatchedMapKey`](https://biomejs.dev/linter/rules/no-mismatched-map-key/), which reports `get()` and `set()` calls that use a different key than the `has()` check before them. Checking one key and then using another is usually a copy-paste mistake.

```js
if (map.has(key)) {
  map.get(otherKey); // reported
}
```
