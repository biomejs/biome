---
"@biomejs/biome": patch
---

Fixed [#12027](https://github.com/biomejs/biome/issues/12027): test-related rules no longer treat method calls on non-identifier expressions as test calls. For example, [`useValidTestTitle`](https://biomejs.dev/linter/rules/use-valid-test-title/) no longer reports `/^#/.test(line)` as a test with an invalid title.
