---
"@biomejs/biome": patch
---

Fixed handling of top-level variables by `useExplicitType` rule ([#5932](https://github.com/biomejs/biome/issues/5932)). Biome now allows all variables with explicit annotations, as well as variables with trivial RHS. Biome no longer emits duplicated errors when an untyped function is assigned to an untyped variable.
