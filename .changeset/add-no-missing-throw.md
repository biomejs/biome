---
"@biomejs/biome": patch
---

Added the nursery rule [`noMissingThrow`](https://biomejs.dev/linter/rules/no-missing-throw/), which reports errors that are created with `new` but never thrown. Its unsafe fix adds the missing `throw` keyword.

```js
function check(value) {
  if (!value) {
    new Error("Missing value");
  }
}
```
