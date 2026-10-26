---
"@biomejs/biome": patch
---

Fixed [#12693](https://github.com/biomejs/biome/issues/12693): the language server no longer leaks memory on configuration reloads. Every time the configuration was reloaded, for example after switching git branches, the daemon kept another copy of every file in the project. Rescanning the project now reuses the files that didn't change, so memory no longer grows with each reload.
