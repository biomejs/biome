---
"@biomejs/biome": patch
---

Fixed [#11704](https://github.com/biomejs/biome/issues/11704): files re-included by negation patterns in a nested `.gitignore` are processed when `vcs.useIgnoreFile` is enabled, even when the ignore file contains `*`.
