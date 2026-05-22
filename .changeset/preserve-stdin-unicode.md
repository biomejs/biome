---
"@biomejs/biome": patch
---

Fixed [#10395](https://github.com/biomejs/biome/issues/10395): `biome format --stdin-file-path` no longer corrupts single-codepoint non-ASCII characters when stdout is redirected.
