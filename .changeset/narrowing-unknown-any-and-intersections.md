---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useStrictBooleanExpressions`](https://biomejs.dev/linter/rules/use-strict-boolean-expressions/) for values of type `unknown`, `any`, generics, and intersections, which now narrow as in TypeScript. For example, `value ?? 1` is now reported after `value != null` when `value` is `unknown`.
