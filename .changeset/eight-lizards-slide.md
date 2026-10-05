---
"@biomejs/biome": minor
---

Added type narrowing for unwritten local `let`/`const` variables and parameters after `typeof`, equality, nullish, and truthiness checks. Type-aware rules now use these refinements in branches, short-circuit expressions, and after early returns, including [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Types such as `{}` no longer imply that a value is always truthy.
