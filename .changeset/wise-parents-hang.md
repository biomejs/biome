---
"@biomejs/biome": patch
---

Fixed [#8314](https://github.com/biomejs/biome/issues/8314): The [`noExtraNonNullAssertion`](https://biomejs.dev/linter/rules/no-extra-non-null-assertion/) rule no longer flags valid uses of non-null assertions in assignment expressions like `a! += b!`.
