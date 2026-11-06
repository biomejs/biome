---
"@biomejs/biome": patch
---

Added the nursery rule [`noNewArray`](https://biomejs.dev/linter/rules/no-new-array/), which reports `new Array()` calls with a single argument. Such a call creates an empty array of the given length when the argument is a number, and an array that holds the argument otherwise.

```js
const array = new Array(length);
```
