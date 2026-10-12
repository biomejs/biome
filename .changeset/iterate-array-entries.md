---
"@biomejs/biome": patch
---

Fixed [#12073](https://github.com/biomejs/biome/issues/12073): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now reports non-exhaustive switches over values read from `for...of` loops over `Array.prototype.entries()`, `Map`, and `Set`, and over properties of array elements indexed by a `number` variable.

[`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) now also reports Promise-returning callbacks passed to array methods such as `find` and `every`.
