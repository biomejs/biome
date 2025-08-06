---
"@biomejs/biome": patch
---

Added the new nursery rule `noUnnecessararyConditions`, which detects whenever some conditions don't
change during the life cycle of the program, and truthy or false, hence deemed redundant.

For example, the following snippets will trigger the rule:

```js
// Always truthy literal conditions
if (true) {
  console.log("always runs");
}
```

```ts
// Unnecessary condition on constrained string type
function foo(arg: 'bar' | 'baz') {
  if (arg) {  // This check is unnecessary
  }
}
```
