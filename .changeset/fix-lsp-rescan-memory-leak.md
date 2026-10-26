---
"@biomejs/biome": patch
---

Fixed [#12693](https://github.com/biomejs/biome/issues/12693): the language server kept another copy of every file in the project each time the configuration was reloaded, for example after switching git branches. Rescanning the project now reuses the files that didn't change.
