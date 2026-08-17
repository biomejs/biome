---
"@biomejs/biome": patch
---

Fixed TypeScript `compilerOptions.paths` resolution when mapping targets omit `./`. Biome now resolves these targets relative to their configured path base.
