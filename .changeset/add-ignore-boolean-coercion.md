---
"@biomejs/biome": patch
---

Added the option `ignoreBooleanCoercion` to [useNullishCoalescing](https://biomejs.dev/linter/rules/use-nullish-coalescing/). When enabled, Biome ignores `||` and `||=` used inside a `Boolean()` call, where coalescing on falsy values is intentional.
