---
"@biomejs/biome": patch
---

Fixed [#12317](https://github.com/biomejs/biome/issues/12317): Biome no longer panics with `path is expected to be under the root` when `vcs.useIgnoreFile` is enabled and a file outside the directory of the root `.gitignore` is checked.
