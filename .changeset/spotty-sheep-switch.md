---
"@biomejs/biome": patch
---

Fixed [#11445](https://github.com/biomejs/biome/issues/11445): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports valid `switch` cases as unreachable when the switched value has a `keyof` type. [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now recognizes known `keyof` property keys when checking switch exhaustiveness.
