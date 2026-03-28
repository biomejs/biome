---
"@biomejs/biome": patch
---

Fixed [`#9475`](https://github.com/biomejs/biome/issues/9475): Fixed a panic when Biome analyzed ambient TypeScript modules containing class constructor, getter, or setter signatures that reference local type aliases. Biome now handles these declarations without crashing during semantic analysis.
