---
"@biomejs/biome": patch
---

Fixed [#11748](https://github.com/biomejs/biome/issues/11748): [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now detects missing cases when switching on properties of elements created by an `Array.from` mapping callback.
