---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) after assignments, which now narrow variables and properties as in TypeScript. For example, `timer?.cancel()` is now reported after `timer = make()` when `make()` returns a `Timer`.
