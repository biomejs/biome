---
"@biomejs/biome": patch
---

[`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now flags missing `true`/`false` cases for `boolean` discriminants, including when `boolean` is a union variant.
