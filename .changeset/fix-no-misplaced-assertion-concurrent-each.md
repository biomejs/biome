---
"@biomejs/biome": patch
---

Fixed [#10635](https://github.com/biomejs/biome/issues/10635): Biome now recognizes chained
table tests such as `test.concurrent.each()` and `it.concurrent.each()` as test calls, fixing
`noMisplacedAssertion` false positives and improving formatting for those test declarations.
