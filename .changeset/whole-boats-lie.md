---
"@biomejs/biome": patch
---

Fixed [#9842](https://github.com/biomejs/biome/issues/9842): [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) no longer skips `||` and `||=` inside a call to a locally declared `Boolean` when `ignoreBooleanCoercion` is enabled.
