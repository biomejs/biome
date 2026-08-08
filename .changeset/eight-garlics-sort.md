---
"@biomejs/biome": patch
---

Fixed [#10139](https://github.com/biomejs/biome/issues/10139): the LSP daemon no longer leaks memory by recompiling the same glob patterns on every configuration reload. Compiled glob matchers are now cached and reused across reloads.
