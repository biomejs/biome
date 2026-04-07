---
"@biomejs/biome": patch
---

`useNullishCoalescing` now also detects ternary expressions that check for `null` or `undefined` and suggests rewriting them with `??`. A new `ignoreTernaryTests` option allows disabling this behavior.
