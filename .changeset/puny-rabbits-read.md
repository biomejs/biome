---
"@biomejs/biome": patch
---

Fixed [#11223](https://github.com/biomejs/biome/issues/11223): Improved the
performance of [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/)
when analyzing async class methods that call other methods through `this`.
