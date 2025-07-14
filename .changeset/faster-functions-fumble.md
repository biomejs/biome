---
"@biomejs/biome": patch
---

Improved [#6172](https://github.com/biomejs/biome/issues/6172): Optimised the way function arguments are stored in Biome's type inference. This led to about 10% performance improvement in `RedisCommander.d.ts` and about 2% on `@next/font` type definitions.
