---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) around function calls: imported, generic, and overloaded type guards narrow, `values.filter(isString)` has the type `string[]`, and a call to a function that returns `never`, such as `fail()`, ends the code path. Functions that can end without returning a value include `undefined` in their return type, as in TypeScript.
