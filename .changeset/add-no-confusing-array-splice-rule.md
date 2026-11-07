---
"@biomejs/biome": patch
---

Added the nursery rule [`noConfusingArraySplice`](https://biomejs.dev/linter/rules/no-confusing-array-splice/), which reports `splice()` and `toSpliced()` calls that replace a single element or insert at index `-1`.

```js
array.splice(index, 1, element); // use `array[index] = element` instead
```
