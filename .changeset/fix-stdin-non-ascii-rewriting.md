---
"@biomejs/biome": patch
---

Fixed [#10395](https://github.com/biomejs/biome/issues/10395): `biome format`, `biome lint` and `biome check` no longer rewrite non-ASCII characters in the source code they print to stdout. Symbols such as `⚠` and `✔` were replaced with `!` and `√` whenever stdout was not a terminal, so piping Biome's output back into a file corrupted it.
