---
"@biomejs/biome": patch
---

Fixed [#10464](https://github.com/biomejs/biome/issues/10464): [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) no longer reports a ternary or `if` statement that checks only `null` or only `undefined` when the type can also be the other, because `??` would change its behavior.
