---
"@biomejs/biome": patch
---

Fixed the safe fix for [`noSkippedTests`](https://biomejs.dev/linter/rules/no-skipped-tests/) so it no longer panics when rewriting skipped test function names such as `xit()`, `xtest()`, and `xdescribe()`.
