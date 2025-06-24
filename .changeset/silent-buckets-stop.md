---
"@biomejs/biome": patch
---

Added new nursery rule [`noUnassignedVariables`](https://biomejs.dev/linter/rules/no-unassigned-variables/), which disallows `let` or `var` variables that are read but never assigned.

The following code is now reported as invalid:

```js
let x;
if (x) {
  console.log(1);
}
```

The following code is now reported as valid:

```js
let x = 1;
if (x) {
  console.log(1);
}
```
