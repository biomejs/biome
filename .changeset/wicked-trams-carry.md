---
"@biomejs/biome": patch
---

Fixed [#11534](https://github.com/biomejs/biome/issues/11534): Biome no longer performs full expression type inference when classifying calls to members whose return type is independent of a generic call argument.

