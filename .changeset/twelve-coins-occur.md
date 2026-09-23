---
"@biomejs/biome": patch
---

Fixed parsing of left shifts between TypeScript instantiation expressions. Biome now parses `f<T> << f<T>` as a left shift of two instantiation expressions, matching TypeScript, instead of reporting a parse error.
