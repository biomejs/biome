---
"@biomejs/biome": patch
---

Added the new nursery rule [`useSetSize`](https://biomejs.dev/linter/rules/use-set-size/), which reports code that copies a `Set` into an array only to read the array's `length`, and suggests reading the `size` property of the `Set` instead.

```js
const uniqueCount = [...new Set(userIds)].length;
```
