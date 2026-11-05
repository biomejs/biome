---
"@biomejs/biome": patch
---

Added the nursery rule [`noArrayDelete`](https://biomejs.dev/linter/rules/no-array-delete/). It reports the `delete` operator used on array elements, which leaves an empty slot in the array instead of removing the element. The rule needs type information, and offers an unsafe fix that replaces the `delete` with a `splice()` call.

```ts
declare const array: number[];
delete array[0];
```
