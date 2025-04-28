---
"@biomejs/biome": minor
---

Biome now prints diagnostics sorted by their severity. The order is the following:
1. information
2. warning
3. error

This means that *error* diagnostics are printed **last**, so users can see them first.
