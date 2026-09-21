---
"@biomejs/biome": patch
---

Fixed [#11748](https://github.com/biomejs/biome/issues/11748): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now reports missing cases for values created with the mapping overload of `Array.from`, including arrays imported from another module.
