---
"@biomejs/biome": patch
---

Type inference is now able to handle ternary conditions in expressions.

## Example

```ts
const condition = Math.random() > -1; // Always true, but dynamic to linter

// We now detect that this may return a `Promise`.
condition ? Promise.reject("ternary bypass") : null;

// On the other hand, we know the following is never a `Promise`:
const alwaysFalsy = 0;
alwaysFalsy ? Promise.reject("ternary bypass") : null;
```
