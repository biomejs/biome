---
"@biomejs/biome": patch
---

Fixed [#12233](https://github.com/biomejs/biome/issues/12233): the `biome` command installed from npm now exits with a non-zero code when the Biome binary is killed by a signal, such as a crash from `SIGABRT` or `SIGSEGV`. Previously, it exited with code `0` and reported the run as successful.
