---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) in `switch` statements, loops, `try` blocks, optional chains, and comparisons with constants. Rules also no longer report `let retries = 3; retries ? a : b` as always truthy, because `retries` is a `number`, as in TypeScript.
