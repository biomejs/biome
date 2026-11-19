---
"@biomejs/biome": patch
---

Fixed [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) reporting `inert || undefined` when `inert` is destructured with a default value, such as `{ inert = false }`.
