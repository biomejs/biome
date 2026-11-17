---
"@biomejs/biome": patch
---

Fixed [#13280](https://github.com/biomejs/biome/issues/13280): the suppression parser no longer panics on a non-ASCII whitespace character after a category. It is accepted as a separator, like an ASCII space.
