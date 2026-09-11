---
"@biomejs/biome": patch
---

Fixed [#11692](https://github.com/biomejs/biome/issues/11692): [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now detects unhandled promises returned through generic method signatures, including Playwright fixtures.
