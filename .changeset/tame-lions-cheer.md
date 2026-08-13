---
"@biomejs/biome": patch
---

Added the `noUselessLengthCheck` rule. It flags length checks that have no effect on the result when paired with `.some()` or `.every()` calls, since those methods already account for the empty-array case. Fixes [#3941](https://github.com/biomejs/biome/issues/3941).
