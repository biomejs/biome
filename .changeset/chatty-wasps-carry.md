---
"@biomejs/biome": patch
---

Fixed [#11475](https://github.com/biomejs/biome/issues/11475): `noUnresolvedImports` no longer reports Bun runtime built-in modules (`bun`, `bun:bundle`, `bun:ffi`, `bun:jsc`, `bun:sqlite`, `bun:test`).
