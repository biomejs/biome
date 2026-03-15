---
"@biomejs/biome": patch
---

Fixed [#9477](https://github.com/biomejs/biome/issues/9477): `source.fixAll.biome` no longer sorts imports when `source.organizeImports.biome` is set to "explicit" or "never" in editor settings. The organize imports action is now excluded from the fix-all pass unless explicitly requested.
