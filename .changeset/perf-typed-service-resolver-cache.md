---
"@biomejs/biome": patch
---

Improved the performance of type-aware lint rules by no longer re-resolving a file's module types for every single expression checked.
