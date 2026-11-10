---
"@biomejs/biome": patch
---

Fixed [#12991](https://github.com/biomejs/biome/issues/12991): the CSS parser no longer panics when a stray `)` appears in the prelude of an `@layer` at-rule (e.g. `@layer)`). The parser now emits a proper parse diagnostic instead of getting stuck.
