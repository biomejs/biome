---
"@biomejs/biome": patch
---

Fixed [#11110](https://github.com/biomejs/biome/issues/11110): `biome lint --watch` and other watch-mode commands no longer emit `internalError/io` diagnostics for file events under `.git/` (such as `.git/index.lock`), and no longer surface transient filesystem errors reported by `notify` for paths that the watcher has no interest in.
