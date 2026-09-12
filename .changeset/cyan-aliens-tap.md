---
"@biomejs/biome": patch
---

Fixed [#11748](https://github.com/biomejs/biome/issues/11748): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now detects missing cases when switching on properties of elements returned by `Array.from`, with or without a mapping callback. Source element types also flow into unannotated mapper parameters.
