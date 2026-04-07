---
"@biomejs/biome": patch
---

Fixed [#9505](https://github.com/biomejs/biome/issues/9505): [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) no longer reports tagged template literals as useless string concatenations. Tagged templates invoke a function and can return non-string values, so combining them with `+` is not equivalent to a single template literal.
