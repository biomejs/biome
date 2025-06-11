---
"@biomejs/biome": patch
---

Fixed [#4665](https://github.com/biomejs/biome/issues/4665): the LSP previously
identified `.cjs` files as ESM files, making rules like `noRedundantUseStrict`
reports incorrectly valid `"use strict"` directives.
