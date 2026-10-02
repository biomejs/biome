---
"@biomejs/biome": patch
---

Fixed [#11944](https://github.com/biomejs/biome/issues/11944): `noUselessStringConcat` safe fix now correctly stringifies number literals using JavaScript's `Number.prototype.toString()` semantics instead of Rust's `f64::to_string()`.
