---
"@biomejs/biome": patch
---

Fixed [#11763](https://github.com/biomejs/biome/issues/11763): TypeScript class members using `override accessor`, such as `override accessor value = 1`, now parse correctly. The reversed order, `accessor override`, now reports that `override` must precede `accessor`.
