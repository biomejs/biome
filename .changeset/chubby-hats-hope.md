---
"@biomejs/biome": patch
---

Fixed an issue where Biome got stuck when analyzing some files. This is usually caused by a bug in the inference engine. Now Biome has some guards in place in case the number of types grows too much, and if that happens, a diagnostic is emitted and the inference is halted.


