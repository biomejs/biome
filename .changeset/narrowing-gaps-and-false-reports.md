---
"@biomejs/biome": patch
---

Improved the accuracy of type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/). They no longer report some checks that are needed, and they catch more checks that can never fail. For example, [`noUnsafeReturn`](https://biomejs.dev/linter/rules/no-unsafe-return/) now reports `return message` inside `catch ({ message }) { ... }`, because `message` has the type `any`.
