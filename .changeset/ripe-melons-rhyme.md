---
"@biomejs/biome": patch
---

Improved [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) to detect conditions that are always truthy because they check built-in global class instances such as `Date`, `Map`, `Set`, `WeakMap`, and `Error`.
