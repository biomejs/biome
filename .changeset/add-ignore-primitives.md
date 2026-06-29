---
"@biomejs/biome": patch
---

Added the option `ignorePrimitives` to [useNullishCoalescing](https://biomejs.dev/linter/rules/use-nullish-coalescing/). When enabled, Biome ignores `||`, `||=`, and ternary expressions whose non-nullish operands are all primitives the option opts out of. Use `true` to ignore all primitives, or an object selecting `string`, `number`, `boolean`, or `bigint`.
