---
"@biomejs/biome": patch
---

Fixed [#12466](https://github.com/biomejs/biome/issues/12466): the Biome daemon used more memory with every `biome check --use-server` run. The daemon now reuses the data that it stores for each file and project, and it keeps syntax trees only for the files that it read most recently.
