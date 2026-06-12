---
"@biomejs/biome": patch
---

Improved the performance of the Biome linter. The improvements are more visible in bigger projects that have more than ~1k files. Early tests showed that in a code base with ~2k files, Biome took less than 26% of time to finish the command.
