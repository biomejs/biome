---
"@biomejs/biome": patch
---

Improved type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) for enums, loose equality such as `value == "a"`, immediately invoked functions, and functions that TypeScript 5.5 infers as type guards, such as `(value: unknown) => typeof value === "string"`. For example, `useExhaustiveSwitchCases` now reports a `switch` on an enum that misses one of its members.
