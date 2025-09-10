---
"@biomejs/biome": patch
---

Improved performance of `noPrivateImports` by eliminating allocations.

In one repository, the total runtime of Biome with only `noPrivateImports` enabled went from ~3.2s down to ~1.4s.
