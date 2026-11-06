---
"@biomejs/biome": patch
---

Fixed [#12991](https://github.com/biomejs/biome/issues/12991): the CSS parser no longer panics when an `@layer` prelude contains `)`, such as `@layer)` or `@layer a b) {}`. It now reports a parse diagnostic instead.
