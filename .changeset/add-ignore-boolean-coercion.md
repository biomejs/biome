---
"@biomejs/biome": patch
---

Added `ignoreBooleanCoercion` to [useNullishCoalescing](https://biomejs.dev/linter/rules/use-nullish-coalescing/), partially addressing [#9232](https://github.com/biomejs/biome/issues/9232). When enabled, Biome ignores `||` and `||=` used inside a `Boolean()` call, where coalescing on falsy values is intentional.
