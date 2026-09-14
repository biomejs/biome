---
"@biomejs/biome": patch
---

Fixed [#7747](https://github.com/biomejs/biome/issues/7747): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now reports missing cases for literal unions derived from const tuples with `(typeof values)[number]` and objects with `keyof typeof object`.

Other type-aware rules, including [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) and [`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/), also recognize supported indexed-access results.
