---
"@biomejs/biome": patch
---

Greatly improved performance of `noImportCycles` by eliminating allocations.

In one repository, the total runtime of Biome with only `noImportCycles` enabled went from ~23s down to ~4s.
