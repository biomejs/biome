---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) for `var` variables, imports, closures, and variables declared without a type, such as `let value;` or `const items = []`. For example, `timer?.cancel()` is now reported in a closure created after checking `timer !== null`.
