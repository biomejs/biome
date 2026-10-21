---
"@biomejs/biome": patch
---

Added the new nursery rule [`useIteratorZip`](https://biomejs.dev/linter/rules/use-iterator-zip/), which reports `for` loops that use an index only to read the same position from several arrays. Such loops can use `Iterator.zip()` instead.

```js
// reported
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
  save(names[index], scores[index]);
}
```
