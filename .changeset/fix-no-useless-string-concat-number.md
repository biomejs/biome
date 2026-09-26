---
"@biomejs/biome": patch
---

Fix `noUselessStringConcat` safe fix to not fold numbers into strings when their serialization diverges between Rust and JavaScript (e.g. for numbers `>= 1e21` or `< 1e-6`).
