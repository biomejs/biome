---
"@biomejs/biome": patch
---

Added the new nursery rule [`noArrayForEach`](https://biomejs.dev/linter/rules/no-array-for-each/), which reports `forEach()` calls that can be written as `for...of` loops. Values known not to be arrays, such as a `Map` or a `Set`, are ignored.

```js
array.forEach((element) => {
	bar(element);
});
```
