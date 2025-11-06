---
"@biomejs/biome": patch
---

Make useArrowFunction fixer safer

Previously, when a function referred to `arguments`,
it could be auto-fixed into an arrow function which would be
broken because `arguments` is not defined for arrow functions.
Now, using `arguments` prevents the fixer from affecting a
function.
