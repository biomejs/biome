---
"@biomejs/biome": patch
---

Fixed [#7525](https://github.com/biomejs/biome/issues/7525): The [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) rule now correctly handles destructured parameters with default values and excludes `undefined` from exhaustiveness checking when a default value is present.