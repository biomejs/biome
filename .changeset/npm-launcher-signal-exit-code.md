---
"@biomejs/biome": patch
---

Fixed [#12233](https://github.com/biomejs/biome/issues/12233): the `biome` command installed from npm now exits with a non-zero code when the Biome binary is terminated by a signal, for example when it crashes. Previously, it exited with code `0`.
