---
"@biomejs/biome": patch
---

Added the nursery rule [`noReturnArrayPush`](https://biomejs.dev/linter/rules/no-return-array-push/), which reports code that uses the return value of `push()` or `unshift()`. These methods return the new length of the array, not the added item or the array.

```js
function add(item) {
  return items.push(item);
}
```
