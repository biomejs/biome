---
"@biomejs/biome": patch
---

Fixed [#3515](https://github.com/biomejs/biome/issues/3515) and [#10395](https://github.com/biomejs/biome/issues/10395), where Biome could corrupt Unicode characters while writing source received through standard input to standard output. Characters such as `⚠` and `✔` are now preserved.
