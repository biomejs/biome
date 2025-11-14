---
"@biomejs/biome": patch
---

Added the nursery rule [`noForIn`](https://biomejs.dev/linter/rules/no-for-in/). Disallow iterating using a for-in loop.

**Invalid:**

```js
for (const i in array) {
  console.log(i, array[i]);
}
```
