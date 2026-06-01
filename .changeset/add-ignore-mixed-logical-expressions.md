---
"@biomejs/biome": patch
---

Added `ignoreMixedLogicalExpressions` to [useNullishCoalescing](https://biomejs.dev/linter/rules/use-nullish-coalescing/), partially addressing [#9232](https://github.com/biomejs/biome/issues/9232). When enabled, Biome ignores `||` and `||=` mixed with `&&` in the same expression tree.
