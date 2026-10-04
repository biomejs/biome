---
"@biomejs/biome": patch
---

Fixed [#12073](https://github.com/biomejs/biome/issues/12073), where [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) missed union cases in `Array.prototype.entries()` loops.
