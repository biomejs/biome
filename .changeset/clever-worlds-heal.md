---
"@biomejs/biome": patch
---

Fixed [#7261](https://github.com/biomejs/biome/issues/7261): two characters `・` (KATAKANA MIDDLE DOT, U+30FB) and `･` (HALFWIDTH KATAKANA MIDDLE DOT, U+FF65) are no longer considered as valid characters in identifiers. Property keys containing these character(s) are now preserved as string literals.
