---
"@biomejs/biome": patch
---

Fixed [#7880](https://github.com/biomejs/biome/issues/7880): [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) no longer reports literal concatenations split across multiple lines when a numeric literal ends the chain.
