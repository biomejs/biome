---
"@biomejs/biome": patch
---

Fixed [#6611](https://github.com/biomejs/biome/issues/6611): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now uses type information to detect more redundant conditions, including `?.`, `??`, `||`, `&&`, comparisons against `null`/`undefined` on non-nullish operands, and `case` clauses that can never match the `switch` value.
