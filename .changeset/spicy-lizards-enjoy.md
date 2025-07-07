---
"@biomejs/biome": patch
---

Fixed a false positive in `noShadow` where a function parameter in a type definition was erroneously flagged as a violation. Fixes [#6038](https://github.com/biomejs/biome/issues/6038).
