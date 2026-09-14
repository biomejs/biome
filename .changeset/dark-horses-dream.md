---
"@biomejs/biome": patch
---

Fixed [#7573](https://github.com/biomejs/biome/issues/7573): added the `requireExplicitCase` option to [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/). When set to `true`, the rule reports missing cases even when the switch has a `default` clause, so you can keep a runtime fallback while checking that every value in the union has its own case. The option defaults to `false`.
