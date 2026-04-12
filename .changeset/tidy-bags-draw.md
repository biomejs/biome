---
"@biomejs/biome": patch
---

Fixed the safe fix for [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) so it no longer panics when rewriting focused test function names such as `fit()` and `fdescribe()`.
