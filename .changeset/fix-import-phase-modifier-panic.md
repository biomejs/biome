---
"@biomejs/biome": patch
---

Fixed [#13178](https://github.com/biomejs/biome/issues/13178): the JS parser no longer panics on `import.defer` or `import.source` when either is parsed as an expression without a following parenthesized argument list. It reports a syntax error instead.
