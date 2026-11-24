---
"@biomejs/biome": patch
---

Added the new nursery rule [`noImmediateMutation`](https://biomejs.dev/linter/rules/no-immediate-mutation/), which reports arrays, objects, `Set`s, and `Map`s that are changed in the statement right after they are created, and variables that are assigned right after they are declared.

```js
const array = [1, 2];
array.push(3); // reported: write `[1, 2, 3]` instead

let foo;
foo = 1; // reported: write `let foo = 1` instead
```
