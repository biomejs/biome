---
"@biomejs/biome": patch
---

Fixed `noUselessElse` safe fix causing syntax errors by withholding the fix when declarations exist in the `else` block (#11937).
