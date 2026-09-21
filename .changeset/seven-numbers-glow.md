---
"@biomejs/biome": patch
---

Fixed [`#9105`](https://github.com/biomejs/biome/issues/9105): `vcs.useIgnoreFile` now evaluates parent directory patterns when matching child paths, preserving re-included directories such as `!/src` while ignoring their excluded siblings.
