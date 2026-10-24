---
"@biomejs/biome": patch
---

Addressed partially [#12466](https://github.com/biomejs/biome/issues/12466): the memory of the Biome daemon grew with every `biome check --use-server` run. The daemon now reuses the data it keeps for each file across runs. In projects that have nested `biome.json` files, or nested ignore files used through `vcs.useIgnoreFile`, the memory used for the settings still grows with each run.
