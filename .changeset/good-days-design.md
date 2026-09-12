---
"@biomejs/biome": patch
---

The rule [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) now also reports `lastIndexOf()` comparisons and `some()` calls with a strict-equality callback.

```js
arr.lastIndexOf(x) !== -1

arr.some(item => item === x)
```
