---
"@biomejs/biome": patch
---

Improved performance of fix-all operations (`--write`). Biome is now smarter when it runs lint rules and assist actions. First, it runs only rules that have code fixes, and then runs the rest of the rules.
