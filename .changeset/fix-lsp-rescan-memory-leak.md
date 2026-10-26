---
"@biomejs/biome": patch
---

Fixed a memory leak in the language server: every time the configuration was reloaded, for example after switching git branches, the daemon kept another copy of every file in the project. Rescanning the project now reuses the files that didn't change, so memory no longer grows with each reload.
