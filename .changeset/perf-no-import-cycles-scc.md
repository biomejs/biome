---
"@biomejs/biome": patch
---

Improved the performance of `noImportCycles` by replacing its per-import graph traversal with a precomputed, cached check for whether an import can possibly be part of a cycle.
