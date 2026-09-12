---
"@biomejs/biome": patch
---

Fixed [#11174](https://github.com/biomejs/biome/issues/11174): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports mutable object properties initialized with literal values as always truthy or falsy.
