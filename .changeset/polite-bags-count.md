---
"@biomejs/biome": patch
---

Extended [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) to report `x ? x : y`, `||=` on properties, `if (x == null)` whatever its type, and values typed `any` or `unknown`; `ignorePrimitives` now ignores a type when any part of it is an ignored primitive.
