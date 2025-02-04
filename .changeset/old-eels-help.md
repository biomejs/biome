---
"@biomejs/biome": patch
---

Fix [#4875](https://github.com/biomejs/biome/issues/4875), where the Jetbrains IDE terminal would output not clickable, relative file path link to the diagnostic file. This does not fix paths without line and column numbers.

