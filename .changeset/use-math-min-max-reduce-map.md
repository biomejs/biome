---
"@biomejs/biome": patch
---

Fixed [#13049](https://github.com/biomejs/biome/issues/13049): `useMathMinMax` no longer flags `.reduce()` calls on string arrays when a string initial value or an identity `.map()` call is used.
