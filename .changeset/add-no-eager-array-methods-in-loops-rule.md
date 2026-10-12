---
"@biomejs/biome": patch
---

Added the new nursery rule [`noEagerArrayMethodsInLoops`](https://biomejs.dev/linter/rules/no-eager-array-methods-in-loops/), which reports `.map()` and `.filter()` calls in the header of `for...of` loops. These calls build a new array that is only used by the loop.

```js
for (const name of users.map((user) => user.name)) {
  console.log(name);
}
```
