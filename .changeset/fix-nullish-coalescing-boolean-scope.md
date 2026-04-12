---
"@biomejs/biome": patch
---

Added `ignoreBooleanCoercion` option to [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/). When enabled, `||` expressions inside `Boolean()` calls are ignored, but only when `Boolean` refers to the global built-in. If `Boolean` is shadowed by a local function, the diagnostic still fires.
