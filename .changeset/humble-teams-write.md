---
"@biomejs/biome": patch
---

Fixed false positives in [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) when numeric cases use different spellings of the same value. For example, `case 0x1` now covers the numeric literal type `1`.
