---
"@biomejs/biome": patch
---

Fixed an issue where Biome got stuck when analyzing some files. This is usually caused by bug in the inference
engine. Now Biome has in place some guards in case the inference engine grows too much, and if that happens,
a diagnostic is emitted and the inference is halted.


