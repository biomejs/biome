---
"@biomejs/biome": patch
---

Fixed [#7020](https://github.com/biomejs/biome/issues/7020): Resolved an issue with analysing types of static member expressions involving unions. If the object type was a union that referenced nested unions, it would trigger an infinite loop as it tried to keep expanding nested unions, and the set of types would grow indefinitely.
