---
"@biomejs/biome": patch
---

Fixed [#11475](https://github.com/biomejs/biome/issues/11475): `noUnresolvedImports` now recognizes Bun runtime built-in modules (`bun`, `bun:test`, `bun:sqlite`, `bun:ffi`, `bun:jsc`, `bun:wrap`) and no longer reports them as unresolved.
