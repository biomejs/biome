---
"@biomejs/biome": patch
---

Fixed [#6611](https://github.com/biomejs/biome/issues/6611): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now uses type information to flag redundant `?.`, `??`, `||`, `&&`, and `null`/`undefined` comparisons on non-nullish operands, reports `!expr` when `expr` is always truthy, and replaces the previous discriminant diagnostic on literal-union `switch` statements with a per-`case` unreachable diagnostic. Example: `function f(x: string) { return x?.length }` is now reported.
