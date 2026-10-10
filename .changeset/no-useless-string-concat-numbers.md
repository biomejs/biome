---
"@biomejs/biome": patch
---

Fixed [#11944](https://github.com/biomejs/biome/issues/11944): [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) safe fix now accurately stringifies numbers according to ECMAScript `Number::toString` semantics, properly handling scientific notation (such as `1e21` and `0.1e-6`), alternate radixes, and separators.
